pub mod app;
pub mod device;
pub mod instance;
pub mod window;

use crate::device::init_device_with_queue;
use crate::instance::init_instance;
use std::sync::Arc;
use vulkano::VulkanLibrary;

use vulkano::memory::allocator::StandardMemoryAllocator;

pub fn main() {
    let library = VulkanLibrary::new().expect("no local Vulkan library/DLL");
    let instance = init_instance(library);
    let (device, queue) = init_device_with_queue(instance);

    let memory_allocator = Arc::new(StandardMemoryAllocator::new_default(device.clone()));
}
