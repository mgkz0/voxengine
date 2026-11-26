use std::sync::Arc;

use crate::backend::{GraphicsBackend, vulkan::VulkanBackend};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

pub struct App {
    pub title: String,
    pub height: u32,
    pub width: u32,
    pub window: Option<Arc<Window>>,
    pub backend: Box<dyn GraphicsBackend>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_title(self.title.as_str())
                        .with_inner_size(LogicalSize::new(self.width, self.height)),
                )
                .unwrap(),
        );
        self.backend.init_from_runtime(&window).unwrap();
        self.window = Some(window);
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        if let WindowEvent::CloseRequested = event {
            event_loop.exit();
        }
    }
}

impl App {
    pub fn new_vulkan(title: String, height: u32, width: u32, backend: VulkanBackend) -> Self {
        Self {
            title,
            height,
            width,
            window: None,
            backend: Box::new(backend),
        }
    }
}
