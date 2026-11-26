use vulkano::{
    image::view::ImageView,
    render_pass::{Framebuffer, FramebufferCreateInfo},
};

use crate::backend::vulkan::VulkanBackend;

impl VulkanBackend {
    pub fn init_framebuffers(&mut self) {
        let render_pass = self.render_pass.as_ref().expect("No render_pass");
        self.framebuffers = self
            .swapchain_images
            .iter()
            .map(|image| {
                let view = ImageView::new_default(image.clone()).unwrap();
                Framebuffer::new(
                    render_pass.clone(),
                    FramebufferCreateInfo {
                        attachments: vec![view],
                        ..Default::default()
                    },
                )
                .unwrap()
            })
            .collect::<Vec<_>>();
    }
}
