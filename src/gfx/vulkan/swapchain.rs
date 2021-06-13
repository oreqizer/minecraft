use ash::extensions::khr::{Surface, Swapchain as AshSwapchain};
use ash::version::{DeviceV1_0};
use ash::{vk, Device, Instance};
use std::sync::Arc;
use winit::dpi::PhysicalSize;

pub struct Swapchain {
    device: Arc<Device>,

    swapchain: vk::SwapchainKHR,
    swapchain_loader: Arc<AshSwapchain>,

    present_images: Vec<vk::Image>,
    present_image_views: Vec<vk::ImageView>,
}

impl Swapchain {
    pub fn builder() -> SwapchainBuilder {
        Default::default()
    }

    pub fn device(&self) -> &Arc<Device> {
        &self.device
    }

    pub fn swapchain(&self) -> vk::SwapchainKHR {
        self.swapchain
    }

    pub fn swapchain_loader(&self) -> &Arc<AshSwapchain> {
        &self.swapchain_loader
    }

    pub fn present_images(&self) -> &Vec<vk::Image> {
        &self.present_images
    }

    pub fn present_image_views(&self) -> &Vec<vk::ImageView> {
        &self.present_image_views
    }
}

impl Drop for Swapchain {
    fn drop(&mut self) {
        unsafe {
            // device.destroy_image_view(self.depth_image_view, None);
            // device.destroy_image(self.depth_image, None);

            for &image_view in self.present_image_views.iter() {
                self.device.destroy_image_view(image_view, None);
            }

            self.swapchain_loader
                .destroy_swapchain(self.swapchain, None);
        }
    }
}

pub struct SwapchainBuilder {
    instance: Option<Arc<Instance>>,
    surface: Option<vk::SurfaceKHR>,
    surface_loader: Option<Arc<Surface>>,

    physical_device: Option<vk::PhysicalDevice>,
    device: Option<Arc<Device>>,
}

impl Default for SwapchainBuilder {
    fn default() -> Self {
        Self {
            instance: None,
            surface: None,
            surface_loader: None,

            physical_device: None,
            device: None,
        }
    }
}

impl SwapchainBuilder {
    pub fn instance(mut self, instance: Arc<Instance>) -> Self {
        self.instance = Some(instance);
        self
    }

    pub fn surface(mut self, surface: vk::SurfaceKHR) -> Self {
        self.surface = Some(surface);
        self
    }

    pub fn surface_loader(mut self, surface_loader: Arc<Surface>) -> Self {
        self.surface_loader = Some(surface_loader);
        self
    }

    pub fn physical_device(mut self, physical_device: vk::PhysicalDevice) -> Self {
        self.physical_device = Some(physical_device);
        self
    }

    pub fn device(mut self, device: Arc<Device>) -> Self {
        self.device = Some(device);
        self
    }

    pub fn build(&mut self, size: &PhysicalSize<u32>) -> Swapchain {
        let instance = self.instance.take().unwrap();
        let surface = self.surface.unwrap();
        let surface_loader = self.surface_loader.take().unwrap();

        let physical_device = self.physical_device.unwrap();
        let device = self.device.take().unwrap();

        let width = size.width;
        let height = size.height;

        unsafe {
            let surface_format = surface_loader
                .get_physical_device_surface_formats(physical_device, surface)
                .unwrap()[0];

            let surface_capabilities = surface_loader
                .get_physical_device_surface_capabilities(physical_device, surface)
                .unwrap();

            let image_count = surface_capabilities.min_image_count + 1;
            let image_count_max = surface_capabilities.max_image_count;

            let image_count = if image_count_max > 0 && image_count > image_count_max {
                image_count_max
            } else {
                image_count
            };

            let surface_resolution = match surface_capabilities.current_extent.width {
                u32::MAX => vk::Extent2D { width, height },
                _ => surface_capabilities.current_extent,
            };

            let pre_transform = if surface_capabilities
                .supported_transforms
                .contains(vk::SurfaceTransformFlagsKHR::IDENTITY)
            {
                vk::SurfaceTransformFlagsKHR::IDENTITY
            } else {
                surface_capabilities.current_transform
            };

            let present_modes = surface_loader
                .get_physical_device_surface_present_modes(physical_device, surface)
                .unwrap();

            let present_mode = present_modes
                .iter()
                .cloned()
                .find(|&mode| mode == vk::PresentModeKHR::MAILBOX)
                .unwrap_or(vk::PresentModeKHR::FIFO);

            let swapchain_loader = AshSwapchain::new(&instance as &Instance, &device as &Device);

            let swapchain_create_info = vk::SwapchainCreateInfoKHR::builder()
                .surface(surface)
                .min_image_count(image_count)
                .image_color_space(surface_format.color_space)
                .image_format(surface_format.format)
                .image_extent(surface_resolution)
                .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
                .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
                .pre_transform(pre_transform)
                .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
                .present_mode(present_mode)
                .clipped(true)
                .image_array_layers(1);

            let swapchain = swapchain_loader
                .create_swapchain(&swapchain_create_info, None)
                .unwrap();

            let present_images = swapchain_loader.get_swapchain_images(swapchain).unwrap();
            let present_image_views: Vec<vk::ImageView> = present_images
                .iter()
                .map(|&image| {
                    let create_view_info = vk::ImageViewCreateInfo::builder()
                        .view_type(vk::ImageViewType::TYPE_2D)
                        .format(surface_format.format)
                        .components(vk::ComponentMapping {
                            r: vk::ComponentSwizzle::R,
                            g: vk::ComponentSwizzle::G,
                            b: vk::ComponentSwizzle::B,
                            a: vk::ComponentSwizzle::A,
                        })
                        .subresource_range(vk::ImageSubresourceRange {
                            aspect_mask: vk::ImageAspectFlags::COLOR,
                            base_mip_level: 0,
                            level_count: 1,
                            base_array_layer: 0,
                            layer_count: 1,
                        })
                        .image(image);
                    device.create_image_view(&create_view_info, None).unwrap()
                })
                .collect();

            Swapchain {
                device: device.clone(),

                swapchain,
                swapchain_loader: Arc::new(swapchain_loader),

                present_images,
                present_image_views,
            }
        }
    }
}
