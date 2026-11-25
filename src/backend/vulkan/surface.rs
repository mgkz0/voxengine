use crate::backend::{GraphicsError, vulkan::VulkanBackend};
use std::sync::Arc;
use vulkano::swapchain::Surface;
use winit::window::Window;

impl VulkanBackend {
    pub fn init_surface(&mut self, window: &Arc<Window>) -> Result<(), GraphicsError> {
        self.surface = Some(
            Surface::from_window(self.instance.clone(), window.clone())
                .map_err(|_| GraphicsError::FailedInit)?,
        );
        Ok(())
    }
}
