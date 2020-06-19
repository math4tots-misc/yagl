use crate::anyhow::{anyhow, Result};

/// WGPU data structures used by Graphics
#[allow(dead_code)]
pub struct Wgpu {
    pub(crate) surface: wgpu::Surface,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) sc_desc: wgpu::SwapChainDescriptor,
    pub(crate) swap_chain: wgpu::SwapChain,
}

impl Wgpu {
    pub(super) async fn new(width: u32, height: u32, surface: wgpu::Surface) -> Result<Self> {
        let adapter = match wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        ).await {
            Some(adapter) => adapter,
            None => return Err(
                anyhow!("Failed to get an adapter for wgpu Surface")
            ),
        };
        let (device, queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        }).await;
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width,
            height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);
        Ok(Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
        })
    }
}
