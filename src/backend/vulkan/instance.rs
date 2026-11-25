use std::sync::Arc;

use crate::backend::GraphicsError;
use crate::backend::vulkan::VulkanBackend;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use winit::event_loop::EventLoop;

impl VulkanBackend {
    pub fn init_instance(event_loop: &EventLoop<()>) -> Result<Arc<Instance>, GraphicsError> {
        let library = VulkanLibrary::new().map_err(|_| GraphicsError::NoLibrary)?;
        let extensions =
            Surface::required_extensions(event_loop).map_err(|_| GraphicsError::FailedInit)?;
        Ok(Instance::new(
            library,
            InstanceCreateInfo {
                flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
                enabled_extensions: extensions,
                ..Default::default()
            },
        )
        .map_err(|_| GraphicsError::FailedInit)?)
    }
}
