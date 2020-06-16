use crate::wgpu;
use crate::winit;
use crate::anyhow::Result;
use crate::anyhow::anyhow;


/// Simplified 2D graphics
#[allow(dead_code)]
pub struct S2D {
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
    sc_desc: wgpu::SwapChainDescriptor,
    swap_chain: wgpu::SwapChain,
}

impl S2D {
    pub async fn from_winit(window: &winit::window::Window) -> Result<Self> {
        let size = window.inner_size();
        let surface = wgpu::Surface::create(window);
        Self::new(size.width, size.height, surface).await
    }
    async fn new(width: u32, height: u32, surface: wgpu::Surface) -> Result<Self> {
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
    pub fn width(&self) -> u32 {
        self.sc_desc.width
    }
    pub fn height(&self) -> u32 {
        self.sc_desc.height
    }
    pub fn resize(&mut self, new_width: u32, new_height: u32) {
        self.sc_desc.width = new_width;
        self.sc_desc.height = new_height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }
}
