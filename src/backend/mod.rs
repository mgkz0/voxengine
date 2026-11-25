use std::sync::Arc;
use thiserror::Error;
use winit::window::Window;

pub mod vulkan;

// later add another backends here like OpenGL

pub enum BackendType {
    Vulkan,
}

#[derive(Debug, Error)]
pub enum GraphicsError {
    #[error("No valid physical device found")]
    NoDevice,
    #[error("No valid graphics queue family found")]
    NoGraphics,
    #[error("Failed to initialize backend")]
    FailedInit,
    #[error("No valid queues found")]
    NoQueues,
    #[error("No library for graphics backend found")]
    NoLibrary,
}

pub trait GraphicsBackend {
    fn init_from_runtime(&mut self, window: &Arc<Window>) -> Result<(), GraphicsError>;
}
