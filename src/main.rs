pub mod app;
pub mod backend;

use crate::{app::App, backend::vulkan::VulkanBackend};
use vulkano::pipeline::graphics::viewport::Viewport;
use winit::event_loop::EventLoop;

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new_vulkan(
        "Engine".to_string(),
        1280,
        720,
        VulkanBackend::new(&event_loop).unwrap(),
    );

    event_loop.run_app(&mut app).unwrap();
}
