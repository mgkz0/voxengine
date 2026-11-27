use std::sync::Arc;

use vulkano::{
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassBeginInfo,
        SubpassContents, SubpassEndInfo,
    },
    image::view::ImageView,
    render_pass::{Framebuffer, FramebufferCreateInfo},
};

use crate::backend::vulkan::{VulkanBackend, framebuffer};

impl VulkanBackend {
    pub fn init_render_pass(&mut self) {
        let device = self.device.clone().expect("No device");
        let swapchain = self.swapchain.as_ref().expect("No surface");
        let render_pass = vulkano::single_pass_renderpass!(
            device,
            attachments: {
                color: {
                    // Set the format the same as the swapchain.
                    format: swapchain.image_format(),
                    samples: 1,
                    load_op: Clear,
                    store_op: Store,
                },
            },
            pass: {
                color: [color],
                depth_stencil: {},
            },
        )
        .unwrap();
        self.render_pass = Some(render_pass)
    }

    pub fn init_framebuffers(&mut self) {
        let render_pass = self.render_pass.as_ref().expect("No render_pass");
        let test = self
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

    pub fn init_clear_command_buffers(&mut self) {
        for framebuffer in &self.framebuffers {
            let mut builder = AutoCommandBufferBuilder::primary(
                self.cbuffer_allocator.as_ref().unwrap().clone(),
                self.queue.as_ref().unwrap().queue_family_index(),
                CommandBufferUsage::OneTimeSubmit,
            )
            .unwrap();
            builder
                .begin_render_pass(
                    RenderPassBeginInfo {
                        clear_values: vec![Some([0.0, 0.0, 1.0, 1.0].into())],
                        ..RenderPassBeginInfo::framebuffer(framebuffer.clone())
                    },
                    SubpassBeginInfo {
                        contents: SubpassContents::Inline,
                        ..Default::default()
                    },
                )
                .unwrap()
                .end_render_pass(SubpassEndInfo::default())
                .unwrap();
            self.command_buffers.push(builder.build().unwrap());
        }
    }
}
