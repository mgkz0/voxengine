pub mod app;
pub mod device;
pub mod instance;

use crate::app::App;
use crate::device::init_device_with_queue;
use crate::instance::init_instance;
use std::sync::Arc;
use vulkano::memory::allocator::StandardMemoryAllocator;
use vulkano::swapchain::Surface;
use winit::event_loop::EventLoop;

pub fn main() {
    let event_loop = EventLoop::new().unwrap();
    let mut app = App::new("Engine".to_string(), 1280, 720);
    let instance = init_instance(&event_loop);
    event_loop.run_app(&mut app).unwrap();
    if let Some(window) = app.window {
        let surface = Surface::from_window(instance.clone(), window.clone()).unwrap();

        let (device, queue) = init_device_with_queue(instance, surface);

        let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
    }
}
