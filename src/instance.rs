use std::sync::Arc;
use vulkano::VulkanLibrary;
use vulkano::instance::{Instance, InstanceCreateFlags, InstanceCreateInfo};
use vulkano::swapchain::Surface;
use winit::event_loop::EventLoop;

pub fn init_instance(event_loop: &EventLoop<()>) -> Arc<Instance> {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let extensions = Surface::required_extensions(event_loop).unwrap();
    Instance::new(
        library,
        InstanceCreateInfo {
            flags: InstanceCreateFlags::ENUMERATE_PORTABILITY,
            enabled_extensions: extensions,
            ..Default::default()
        },
    )
    .expect("failed to create instance")
}
