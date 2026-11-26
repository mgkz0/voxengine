pub mod device;
pub mod framebuffer;
pub mod instance;
pub mod render_pass;
pub mod surface;
pub mod swapchain;

use crate::backend::{GraphicsBackend, GraphicsError};
use std::sync::Arc;
use vulkano::{
    device::{Device, Queue},
    image::Image,
    instance::Instance,
    render_pass::{Framebuffer, RenderPass},
    swapchain::{Surface, Swapchain},
};
use winit::event_loop::EventLoop;
use winit::window::Window;

pub struct VulkanBackend {
    instance: Arc<Instance>,
    device: Option<Arc<Device>>,
    surface: Option<Arc<Surface>>,
    queue: Option<Arc<Queue>>,
    swapchain: Option<Arc<Swapchain>>,
    swapchain_images: Vec<Arc<Image>>,
    render_pass: Option<Arc<RenderPass>>,
    framebuffers: Vec<Arc<Framebuffer>>,
}

impl GraphicsBackend for VulkanBackend {
    fn init_from_runtime(&mut self, window: &Arc<Window>) -> Result<(), GraphicsError> {
        //TODO add reasanoble error handle messages
        self.init_surface(window)?;
        self.init_device_with_queue()?;
        self.init_swapchain(window); // TODO add error handling to swapchain initialization
        self.init_render_pass();
        self.init_framebuffers();
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
            swapchain: None,
            swapchain_images: Vec::new(),
            render_pass: None,
        })
    }
}
