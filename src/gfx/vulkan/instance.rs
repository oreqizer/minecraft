use std::borrow::Cow;
use std::ffi::{CStr, CString};

use ash::extensions::ext::DebugUtils;
use ash::extensions::khr::Surface;
use ash::version::{EntryV1_0, InstanceV1_0};
use ash::{vk, Entry, Instance as AshInstance};
use winit::event_loop::EventLoop;
use winit::window::Window;
use winit::{dpi::LogicalSize, window::WindowBuilder};
use winit::dpi::PhysicalSize;

pub struct Instance {
    window: Window,

    instance: AshInstance,
    debug_utils_loader: DebugUtils,
    debug_callback: vk::DebugUtilsMessengerEXT,

    surface: vk::SurfaceKHR,
    surface_loader: Surface,
}

impl Instance {
    pub fn new<T>(event_loop: &EventLoop<T>) -> Self {
        unsafe {
            const INIT_WIDTH: u32 = 800;
            const INIT_HEIGHT: u32 = 600;
            const TITLE: &str = "Minecraft";

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
                .pfn_user_callback(Some(vulkan_debug_callback));

            let debug_utils_loader = DebugUtils::new(&entry, &instance);
            let debug_callback = debug_utils_loader
                .create_debug_utils_messenger(&debug_info, None)
                .unwrap();

            let surface = ash_window::create_surface(&entry, &instance, &window, None).unwrap();
            let surface_loader = Surface::new(&entry, &instance);

            Self {
                window,
                instance,
                debug_utils_loader,
                debug_callback,
                surface,
                surface_loader,
            }
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.surface_loader.destroy_surface(self.surface, None);
            self.debug_utils_loader
                .destroy_debug_utils_messenger(self.debug_callback, None);
            self.instance.destroy_instance(None);
        }
    }

    pub fn instance(&self) -> &AshInstance {
        &self.instance
    }

    pub fn surface(&self) -> vk::SurfaceKHR {
        self.surface
    }

    pub fn surface_loader(&self) -> &Surface {
        &self.surface_loader
    }

    pub fn window_size(&self) -> PhysicalSize<u32> {
        self.window.inner_size()
    }
}

// https://github.com/MaikKlein/ash/blob/master/examples/src/lib.rs#L87
unsafe extern "system" fn vulkan_debug_callback(
    message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    _user_data: *mut std::os::raw::c_void,
) -> vk::Bool32 {
    let callback_data = *p_callback_data;
    let message_id_number: i32 = callback_data.message_id_number as i32;

    let message_id_name = if callback_data.p_message_id_name.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message_id_name).to_string_lossy()
    };

    let message = if callback_data.p_message.is_null() {
        Cow::from("")
    } else {
        CStr::from_ptr(callback_data.p_message).to_string_lossy()
    };

    println!(
        "{:?}:\n{:?} [{} ({})] : {}\n",
        message_severity,
        message_type,
        message_id_name,
        &message_id_number.to_string(),
        message,
    );

    vk::FALSE
}
