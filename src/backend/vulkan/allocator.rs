use std::sync::Arc;
use vulkano::memory::allocator::StandardMemoryAllocator;
use crate::backend::vulkan::VulkanBackend;

impl VulkanBackend {
    pub fn init_allocator(&mut self) {
        let device = self.device.as_ref().expect("No device");
        self.allocator = Some(Arc::new(StandardMemoryAllocator::new_default(device.clone())));
    }
}

