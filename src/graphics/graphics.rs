use crate::wgpu;
use crate::winit;
use crate::anyhow::Result;
use crate::anyhow::anyhow;
use crate::Game;
use crate::AppContext;
use crate::DrawTask;


/// Simplified 2D graphics
#[allow(dead_code)]
pub struct Graphics {
    pub(crate) surface: wgpu::Surface,
    pub(crate) adapter: wgpu::Adapter,
    pub(crate) device: wgpu::Device,
    pub(crate) queue: wgpu::Queue,
    pub(crate) sc_desc: wgpu::SwapChainDescriptor,
    pub(crate) swap_chain: wgpu::SwapChain,
}

impl Graphics {
    pub(crate) async fn from_winit(window: &winit::window::Window) -> Result<Self> {
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

    /// Called by yagl code to adjust the graphics when the window changes
    /// Should not need to be called by client code
    pub(crate) fn resize(&mut self, new_width: u32, new_height: u32) {
        self.sc_desc.width = new_width;
        self.sc_desc.height = new_height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    pub(crate) fn render<G: Game>(&mut self, game: &mut G, actx: &mut AppContext) -> Result<()> {
        let frame = match self.swap_chain.get_next_texture() {
            Ok(frame) => frame,
            Err(error) => return Err(anyhow!(format!("{:?}", error))),
        };

        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        let gctx = GraphicsContext {
            graphics: self,
        };

        let draw_tasks = game.draw(actx, gctx)?;

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[
                    wgpu::RenderPassColorAttachmentDescriptor {
                        attachment: &frame.view,
                        resolve_target: None,
                        load_op: wgpu::LoadOp::Clear,
                        store_op: wgpu::StoreOp::Store,
                        clear_color: wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        },
                    },
                ],
                depth_stencil_attachment: None,
            });

            for draw_task in draw_tasks.iter() {
                match draw_task {
                    DrawTask::SetPipeline(pipeline) => {
                        render_pass.set_pipeline(pipeline);
                    }
                    DrawTask::SetVertexBuffer{ slot, buffer, offset, size } => {
                        render_pass.set_vertex_buffer(*slot, buffer, *offset, *size);
                    }
                    DrawTask::Draw { vertices, instances } => {
                        render_pass.draw(vertices.clone(), instances.clone());
                    }
                }
            }
        }

        self.queue.submit(&[encoder.finish()]);

        Ok(())
    }
}

/// Context that's available in any place where rendering/drawing is needed
pub struct GraphicsContext<'a> {
    pub(crate) graphics: &'a mut Graphics,
}