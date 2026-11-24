use std::sync::Arc;
use vulkano::device::physical::PhysicalDevice;
use vulkano::device::{Device, DeviceCreateInfo, QueueCreateInfo, QueueFlags};
use vulkano::device::{DeviceExtensions, Queue};
use vulkano::instance::Instance;
use vulkano::swapchain::Surface;

fn select_physical_device(
    instance: Arc<Instance>,
    device_extensions: &DeviceExtensions,
) -> Arc<PhysicalDevice> {
    instance
        .enumerate_physical_devices()
        .expect("could not enumerate devices")
        .find(|device| device.supported_extensions().contains(device_extensions))
        .unwrap()
}

fn select_queue_index(physical_device: &Arc<PhysicalDevice>, surface: &Arc<Surface>) -> u32 {
    physical_device
        .queue_family_properties()
        .iter()
        .enumerate()
        .position(|(idx, properties)| {
            properties.queue_flags.contains(QueueFlags::GRAPHICS)
                && physical_device
                    .surface_support(idx as u32, surface)
                    .unwrap()
        })
        .expect("no suitable queue family found") as u32
}

fn build_logical_device_with_queue(
    physical_device: Arc<PhysicalDevice>,
    device_extensions: DeviceExtensions,
    queue_family_index: u32,
) -> (Arc<Device>, Arc<Queue>) {
    let (device, mut queues) = Device::new(
        physical_device,
        DeviceCreateInfo {
            queue_create_infos: vec![QueueCreateInfo {
                queue_family_index,
                ..Default::default()
            }],
            enabled_extensions: device_extensions,
            ..Default::default()
        },
    )
    .expect("failed to create device");
    (device, queues.next().unwrap())
}

pub fn init_device_with_queue(
    instance: Arc<Instance>,
    surface: Arc<Surface>,
) -> (Arc<Device>, Arc<Queue>) {
    let device_extensions = DeviceExtensions {
        khr_swapchain: true,
        ..Default::default()
    };
    let physical_device = select_physical_device(instance, &device_extensions);
    let queue_family_index = select_queue_index(&physical_device, &surface);
    build_logical_device_with_queue(physical_device, device_extensions, queue_family_index)
}
