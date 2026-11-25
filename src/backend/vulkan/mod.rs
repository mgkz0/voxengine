pub mod device;
pub mod instance;
pub mod surface;

use crate::backend::{GraphicsBackend, GraphicsError};
use std::sync::Arc;
use vulkano::{
    device::{Device, Queue},
    instance::Instance,
    swapchain::Surface,
};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct VulkanBackend {
    instance: Arc<Instance>,
    device: Option<Arc<Device>>,
    surface: Option<Arc<Surface>>,
    queue: Option<Arc<Queue>>,
}

impl GraphicsBackend for VulkanBackend {
    fn init_from_runtime(&mut self, window: &Arc<Window>) -> Result<(), GraphicsError> {
        self.init_surface(window)?;
        self.init_device_with_queue()?;
        Ok(())
    }
}

impl VulkanBackend {
    pub fn new(event_loop: &EventLoop<()>) -> Result<Self, GraphicsError> {
        let instance = Self::init_instance(event_loop)?;
        Ok(Self {
            instance: instance,
            device: None,
            surface: None,
            queue: None,
        })
    }
}
