use crate::backend::vulkan::VulkanBackend;

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
}
