use vulkano::{buffer::BufferContents, pipeline::graphics::vertex_input::Vertex};
use crate::backend::vulkan::VulkanBackend;


#[derive(BufferContents, Vertex)]
#[repr(C)]
pub struct V {
    #[format(R32G32_SFLOAT)]
    position: [f32; 2],
}


impl V {
    pub fn new(x: f32, y: f32) -> Self {
        Self { position: [x, y] }
    }
}

mod vs {
    vulkano_shaders::shader! {
        ty: "vertex",
        src: "
            #version 460

            layout(location = 0) in vec2 position;

            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
            }
        ",
    }
}

mod fs {
    vulkano_shaders::shader! {
        ty: "fragment",
        src: "
            #version 460

            layout(location = 0) out vec4 f_color;

            void main() {
                f_color = vec4(1.0, 0.0, 0.0, 1.0);
            }
        ",
    }
}


impl VulkanBackend {
    pub fn init_vertex_buffer(&mut self) {

    }
}
