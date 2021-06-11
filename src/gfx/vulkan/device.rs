use ash::extensions::khr::Swapchain;
use ash::version::{DeviceV1_0, InstanceV1_0};
use ash::{vk, Device as AshDevice};

use crate::gfx::vulkan::Instance;

pub struct Device {
    physical_device: vk::PhysicalDevice,
    device: AshDevice,
    queue: vk::Queue,
}

impl Device {
    pub fn new(instance: &Instance) -> Self {
        let surface = instance.surface();
        let surface_loader = instance.surface_loader();

        unsafe {
            let physical_devices = instance
                .instance()
                .enumerate_physical_devices()
                .expect("failed to enumerate Vulkan physical devices");

            let (physical_device, queue_family_index) = physical_devices
                .iter()
                .map(|pdevice| {
                    instance
                        .instance()
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

            let device_extension_names = [Swapchain::name().as_ptr()];
            let features = vk::PhysicalDeviceFeatures {
                shader_clip_distance: 1,
                ..Default::default()
            };

            let device_create_info = vk::DeviceCreateInfo::builder()
                .queue_create_infos(&queue_info)
                .enabled_extension_names(&device_extension_names)
                .enabled_features(&features);

            let device = instance
                .instance()
                .create_device(physical_device, &device_create_info, None)
                .unwrap();

            let queue = device.get_device_queue(queue_family_index as u32, 0);

            Self {
                physical_device,
                device,
                queue,
            }
        }
    }

    pub fn destroy(&mut self) {
        unsafe {
            self.device.destroy_device(None);
        }
    }

    pub fn physical_device(&self) -> vk::PhysicalDevice {
        self.physical_device
    }

    pub fn device(&self) -> &AshDevice {
        &self.device
    }

    pub fn queue(&self) -> &vk::Queue {
        &self.queue
    }
}
