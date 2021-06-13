use std::ffi::CString;
use std::io::Cursor;
use std::mem::ManuallyDrop;
use std::sync::Arc;

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::{Surface, Swapchain as AshSwapchain};
use ash::util::read_spv;
use ash::version::{DeviceV1_0, EntryV1_0, InstanceV1_0};
use ash::{vk, Device, Entry, Instance};
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::{dpi::LogicalSize, window::WindowBuilder};

use super::debug;
use super::swapchain::Swapchain;

pub struct Vulkan {
    window: Window,

    instance: Arc<Instance>,
    debug_utils_loader: DebugUtils,
    debug_callback: vk::DebugUtilsMessengerEXT,

    surface: vk::SurfaceKHR,
    surface_format: vk::SurfaceFormatKHR,
    surface_loader: Arc<Surface>,

    physical_device: vk::PhysicalDevice,
    device: Arc<Device>,
    present_queue: vk::Queue,

    swapchain: ManuallyDrop<Arc<Swapchain>>,
}

impl Vulkan {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        const INIT_WIDTH: u32 = 800;
        const INIT_HEIGHT: u32 = 600;
        const TITLE: &str = "Minecraft";

        // TODO make a "build" function for things that need rebuilding on resize
        // that takes width and height
        unsafe {
            // === INSTANCE ===

            let entry = Entry::new().unwrap();
            let window = WindowBuilder::new()
                .with_title(TITLE)
                .with_inner_size(LogicalSize::new(
                    f64::from(INIT_WIDTH),
                    f64::from(INIT_HEIGHT),
                ))
                .build(event_loop)
                .unwrap();

            let layer_names = [CString::new("VK_LAYER_KHRONOS_validation").unwrap()];
            let layer_names: Vec<*const i8> = layer_names
                .iter()
                .map(|raw_name| raw_name.as_ptr())
                .collect();

            let extension_names = ash_window::enumerate_required_extensions(&window).unwrap();
            let mut extension_names = extension_names
                .iter()
                .map(|ext| ext.as_ptr())
                .collect::<Vec<_>>();
            extension_names.push(DebugUtils::name().as_ptr());

            let app_name = CString::new(TITLE).unwrap();
            let app_info = vk::ApplicationInfo::builder()
                .application_name(&app_name)
                .engine_name(&app_name)
                .api_version(vk::make_version(1, 2, 0));

            let create_info = vk::InstanceCreateInfo::builder()
                .application_info(&app_info)
                .enabled_layer_names(&layer_names)
                .enabled_extension_names(&extension_names);

            let instance = entry
                .create_instance(&create_info, None)
                .expect("failed to create Vulkan instance");

            let debug_info = vk::DebugUtilsMessengerCreateInfoEXT::builder()
                .message_severity(
                    vk::DebugUtilsMessageSeverityFlagsEXT::ERROR
                        | vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                        | vk::DebugUtilsMessageSeverityFlagsEXT::INFO,
                )
                .message_type(vk::DebugUtilsMessageTypeFlagsEXT::all())
                .pfn_user_callback(Some(debug::vulkan_debug_callback));

            let debug_utils_loader = DebugUtils::new(&entry, &instance);
            let debug_callback = debug_utils_loader
                .create_debug_utils_messenger(&debug_info, None)
                .unwrap();

            let surface = ash_window::create_surface(&entry, &instance, &window, None).unwrap();
            let surface_loader = Surface::new(&entry, &instance);

            // === DEVICE ===

            let physical_devices = instance
                .enumerate_physical_devices()
                .expect("failed to enumerate Vulkan physical devices");

            let (physical_device, queue_family_index) = physical_devices
                .iter()
                .map(|pdevice| {
                    instance
                        .get_physical_device_queue_family_properties(*pdevice)
                        .iter()
                        .enumerate()
                        .filter_map(|(index, ref info)| {
                            let supports_graphics =
                                info.queue_flags.contains(vk::QueueFlags::GRAPHICS);
                            let supports_surface = surface_loader
                                .get_physical_device_surface_support(
                                    *pdevice,
                                    index as u32,
                                    surface,
                                )
                                .unwrap();

                            if supports_graphics && supports_surface {
                                Some((*pdevice, index as u32))
                            } else {
                                None
                            }
                        })
                        .next()
                })
                .flatten()
                .next()
                .expect("no suitable Vulkan physical device");

            let priorities = [1.0];
            let queue_info = [vk::DeviceQueueCreateInfo::builder()
                .queue_family_index(queue_family_index)
                .queue_priorities(&priorities)
                .build()];

            let device_extension_names = [AshSwapchain::name().as_ptr()];
            let features = vk::PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
            };

            let device_create_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_info)
                .enabled_extension_names(&device_extension_names)
                .enabled_features(&features);

            let device = instance
                .create_device(physical_device, &device_create_info, None)
                .unwrap();

            let present_queue = device.get_device_queue(queue_family_index as u32, 0);

            let surface_format = surface_loader
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap()[0];

            // === SWAPCHAIN ===

            let instance = Arc::new(instance);
            let surface_loader = Arc::new(surface_loader);
            let device = Arc::new(device);

            let swapchain = ManuallyDrop::new(Arc::new(
                Swapchain::builder()
                    .instance(instance.clone())
                    .surface(surface)
                    .surface_loader(surface_loader.clone())
                    .physical_device(physical_device)
                    .device(device.clone())
                    .build(&window.inner_size()),
            ));

            Self {
                window,

                instance,
                debug_utils_loader,
                debug_callback,

                surface,
                surface_format,
                surface_loader,

                physical_device,
                device,
                present_queue,

                swapchain,
            }
        }
    }

    pub fn clone_device(&self) -> Arc<Device> {
        self.device.clone()
    }

    pub fn surface_format(&self) -> vk::SurfaceFormatKHR {
        self.surface_format
    }

    pub fn create_shader_module(&self, file: &mut Cursor<&[u8]>) -> vk::ShaderModule {
        let code = read_spv(file).unwrap();

        let create_info = vk::ShaderModuleCreateInfo::builder().code(&code);

        unsafe {
            self.device
                .create_shader_module(&create_info, None)
                .unwrap()
        }
    }

    pub fn recreate_swapchain(&mut self) {
        unsafe {
            self.device.device_wait_idle();

            ManuallyDrop::drop(&mut self.swapchain);

            self.swapchain = ManuallyDrop::new(Arc::new(
                Swapchain::builder()
                    .instance(self.instance.clone())
                    .surface(self.surface)
                    .surface_loader(self.surface_loader.clone())
                    .physical_device(self.physical_device)
                    .device(self.device.clone())
                    .build(&self.window.inner_size()),
            ));
        }
    }
}

impl Drop for Vulkan {
    fn drop(&mut self) {
        unsafe {
            ManuallyDrop::drop(&mut self.swapchain);

            self.device.destroy_device(None);

            self.surface_loader.destroy_surface(self.surface, None);
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_callback, None);
            self.instance.destroy_instance(None);
        }
    }
}
