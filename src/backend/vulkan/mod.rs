pub mod allocator;
pub mod device;
pub mod framebuffer;
pub mod instance;
pub mod pipeline;
pub mod shaders;
pub mod surface;
pub mod swapchain;
pub mod vertex;

use crate::backend::{GraphicsBackend, GraphicsError};
use std::sync::Arc;
use vulkano::command_buffer::PrimaryAutoCommandBuffer;
use vulkano::command_buffer::allocator::StandardCommandBufferAllocator;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::pipeline::GraphicsPipeline;
use vulkano::pipeline::graphics::viewport::Viewport;
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
    mem_allocator: Option<Arc<StandardMemoryAllocator>>,
    cbuffer_allocator: Option<Arc<StandardCommandBufferAllocator>>,
    command_buffers: Vec<Arc<PrimaryAutoCommandBuffer>>,
    graphics_pipeline: Option<Arc<GraphicsPipeline>>,
    viewport: Viewport,
}

impl GraphicsBackend for VulkanBackend {
    fn init_from_runtime(&mut self, window: &Arc<Window>) -> Result<(), GraphicsError> {
        //TODO add error handling for potential panics
        self.init_surface(window)?;
        self.init_device_with_queue()?;
        self.init_swapchain(window);
        self.init_render_pass();
        self.init_framebuffers();
        self.init_allocator();
        self.init_command_buffer_allocator();
        self.init_clear_command_buffers();
        Ok(())
    }
}

impl VulkanBackend {
    pub fn new(event_loop: &EventLoop<()>, height: f32, width: f32) -> Result<Self, GraphicsError> {
        let instance = Self::init_instance(event_loop)?;
        let viewport = Viewport {
            offset: [0.0, 0.0],
            extent: [width, height],
            depth_range: 0.0..=1.0,
        };
        Ok(Self {
            instance: instance,
            device: None,
            surface: None,
            queue: None,
            swapchain: None,
            swapchain_images: Vec::new(),
            render_pass: None,
            framebuffers: Vec::new(),
            mem_allocator: None,
            cbuffer_allocator: None,
            command_buffers: Vec::new(),
            graphics_pipeline: None,
            viewport,
        })
    }
}
