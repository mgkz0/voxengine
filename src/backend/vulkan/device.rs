use crate::backend::GraphicsError;
use crate::backend::vulkan::VulkanBackend;
use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::device::{DeviceExtensions, Queue};

impl VulkanBackend {
    fn select_physical_device(
        &self,
        device_extensions: &DeviceExtensions,
    ) -> Option<Arc<PhysicalDevice>> {
        Some(
            self.instance
                .enumerate_physical_devices()
                .ok()?
                .find(|device| device.supported_extensions().contains(device_extensions))?,
        )
    }

    fn select_queue_index(&self, physical_device: &Arc<PhysicalDevice>) -> Option<u32> {
        let surface = self.surface.as_ref()?;
        let idx = physical_device
            .queue_family_properties()
            .iter()
            .enumerate()
            .position(|(idx, properties)| {
                properties.queue_flags.contains(QueueFlags::GRAPHICS)
                    && physical_device.surface_support(idx as u32, surface).is_ok()
            })?;
        Some(idx as u32)
    }

    fn build_logical_device_with_queue(
        &self,
        physical_device: Arc<PhysicalDevice>,
        device_extensions: DeviceExtensions,
        queue_family_index: u32,
    ) -> Result<(Arc<Device>, Arc<Queue>), GraphicsError> {
        let (device, mut queues) = Device::new(
            physical_device,
            DeviceCreateInfo {
                queue_create_infos: vec![QueueCreateInfo {
                    queue_family_index,
                    ..Default::default()
                }],
                enabled_extensions: device_extensions,
                ..Default::default()
            },
        )
        .map_err(|_| GraphicsError::FailedInit)?;
        let queue = queues.next().ok_or(GraphicsError::NoQueues)?;
        Ok((device, queue))
    }

    pub fn init_device_with_queue(&mut self) -> Result<(), GraphicsError> {
        let device_extensions = DeviceExtensions {
            khr_swapchain: true,
            ..Default::default()
        };
        let physical_device = self
            .select_physical_device(&device_extensions)
            .ok_or(GraphicsError::NoDevice)?;
        let queue_family_index = self
            .select_queue_index(&physical_device)
            .ok_or(GraphicsError::NoGraphics)?;
        let (device, queue) = self.build_logical_device_with_queue(
            physical_device,
            device_extensions,
            queue_family_index,
        )?;
        self.device = Some(device);
        self.queue = Some(queue);
        Ok(())
    }
}
