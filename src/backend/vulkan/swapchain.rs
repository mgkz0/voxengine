use std::sync::Arc;

use crate::backend::vulkan::VulkanBackend;
use vulkano::image::ImageUsage;
use vulkano::swapchain::{Swapchain, SwapchainCreateInfo};
use winit::window::Window;

impl VulkanBackend {
    pub fn init_swapchain(&mut self, window: &Arc<Window>) {
        let device = self.device.as_ref().expect("No device");
        let surface = self.surface.as_ref().expect("No surface");
        let dims = window.inner_size();
        let phys_device = device.physical_device();

        let caps = phys_device
            .surface_capabilities(surface, Default::default())
            .unwrap();
        let composite_alpha = caps.supported_composite_alpha.into_iter().next().unwrap();
        let image_format = phys_device
            .surface_formats(surface, Default::default())
            .unwrap()[0]
            .0;

        let (swapchain, images) = Swapchain::new(
            device.clone(),
            surface.clone(),
            SwapchainCreateInfo {
                min_image_count: caps.min_image_count + 1,
                image_format,
                image_extent: dims.into(),
                image_usage: ImageUsage::COLOR_ATTACHMENT,
                composite_alpha,
                ..Default::default()
            },
        )
        .unwrap();
        self.swapchain = Some(swapchain);
        self.swapchain_images = images
    }
}
