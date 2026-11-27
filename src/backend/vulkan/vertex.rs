use crate::backend::vulkan::VulkanBackend;
use vulkano::{
    buffer::{BufferContents, BufferUsage, Subbuffer},
    pipeline::graphics::vertex_input::Vertex,
};

#[derive(BufferContents, Vertex, Clone, Copy)]
#[repr(C)]
pub struct V {
    #[format(R32G32B32_SFLOAT)]
    position: [f32; 3],
}

impl V {
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self {
            position: [x, y, z],
        }
    }
}

pub struct Mesh {
    pub vertex: Subbuffer<[V]>,
    pub index: Option<Subbuffer<[u32]>>,
}

impl Mesh {
    pub fn new(vertex: Subbuffer<[V]>, index: Option<Subbuffer<[u32]>>) -> Self {
        Self { vertex, index }
    }
}

impl VulkanBackend {
    pub fn build_mesh(&self, vertices: Vec<V>, indices: Option<Vec<u32>>) -> Mesh {
        let vertex_buffer = self.build_buffer(vertices, BufferUsage::VERTEX_BUFFER);
        let index_buffer = match indices {
            Some(indices) => Some(self.build_buffer(indices, BufferUsage::INDEX_BUFFER)),
            None => None,
        };

        Mesh::new(vertex_buffer, index_buffer)
    }
}
