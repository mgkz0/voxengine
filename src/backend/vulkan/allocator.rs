use crate::backend::vulkan::VulkanBackend;
use std::sync::Arc;
use vulkano::{
    buffer::{Buffer, BufferContents, BufferCreateInfo, BufferUsage, Subbuffer},
    command_buffer::allocator::StandardCommandBufferAllocator,
    memory::allocator::{AllocationCreateInfo, MemoryTypeFilter, StandardMemoryAllocator},
};

impl VulkanBackend {
    pub fn init_allocator(&mut self) {
        let device = self.device.as_ref().expect("No device");
        self.mem_allocator = Some(Arc::new(StandardMemoryAllocator::new_default(
            device.clone(),
        )));
    }

    pub fn build_buffer<T>(&self, data: Vec<T>, usage: BufferUsage) -> Subbuffer<[T]>
    where
        T: BufferContents,
    {
        let allocator = self.mem_allocator.as_ref().expect("No allocator");
        let create_info = BufferCreateInfo {
            usage,
            ..Default::default()
        };
        let alloc_info = AllocationCreateInfo {
            memory_type_filter: MemoryTypeFilter::PREFER_DEVICE
                | MemoryTypeFilter::HOST_SEQUENTIAL_WRITE,
            ..Default::default()
        };
        Buffer::from_iter(allocator.clone(), create_info, alloc_info, data).unwrap()
    }

    pub fn init_command_buffer_allocator(&mut self) {
        self.cbuffer_allocator = Some(Arc::new(StandardCommandBufferAllocator::new(
            self.device.as_ref().unwrap().clone(),
            Default::default(),
        )));
    }
}
