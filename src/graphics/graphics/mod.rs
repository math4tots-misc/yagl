mod ms;
mod wg;
use crate::wgpu;
use crate::winit;
use crate::anyhow::Result;
use crate::anyhow::anyhow;
use crate::Game;
use crate::AppContext;
use crate::DrawTask;
use wg::Wgpu;
use ms::MeshStuff;

pub(crate) use ms::PreparedMesh;


/// Simplified 2D graphics
#[allow(dead_code)]
pub struct Graphics {
    pub(crate) wgpu: Wgpu,
    pub(crate) mesh: MeshStuff,
}

impl Graphics {
    pub(crate) async fn from_winit(window: &winit::window::Window) -> Result<Self> {
        let size = window.inner_size();
        let surface = wgpu::Surface::create(window);
        Self::new(size.width, size.height, surface).await
    }
    async fn new(width: u32, height: u32, surface: wgpu::Surface) -> Result<Self> {
        let wgpu = Wgpu::new(width, height, surface).await?;
        let mesh = MeshStuff::new(&wgpu)?;
        Ok(Self { wgpu, mesh })
    }
    pub fn width(&self) -> u32 {
        self.wgpu.sc_desc.width
    }
    pub fn height(&self) -> u32 {
        self.wgpu.sc_desc.height
    }

    /// Called by yagl code to adjust the graphics when the window changes
    /// Should not need to be called by client code
    pub(crate) fn resize(&mut self, new_width: u32, new_height: u32) {
        self.wgpu.sc_desc.width = new_width;
        self.wgpu.sc_desc.height = new_height;
        self.wgpu.swap_chain = self.wgpu.device.create_swap_chain(&self.wgpu.surface, &self.wgpu.sc_desc);
    }

    pub(crate) fn render<G: Game>(&mut self, game: &mut G, actx: &mut AppContext) -> Result<()> {
        let frame = match self.wgpu.swap_chain.get_next_texture() {
            Ok(frame) => frame,
            Err(error) => return Err(anyhow!(format!("{:?}", error))),
        };

        let gctx = GraphicsContext {
            graphics: self,
        };

        let draw_tasks = game.draw(actx, gctx)?;

        let mut encoder = self.wgpu.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

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
                    DrawTask::SetVertexBuffer { slot, buffer, offset, size } => {
                        render_pass.set_vertex_buffer(*slot, buffer, *offset, *size);
                    }
                    DrawTask::SetIndexBuffer { buffer, offset, size } => {
                        render_pass.set_index_buffer(buffer, *offset, *size);
                    }
                    DrawTask::SetBindGroup { index, bind_group, offsets } => {
                        render_pass.set_bind_group(*index, bind_group, offsets)
                    }
                    DrawTask::Draw { vertices, instances } => {
                        render_pass.draw(vertices.clone(), instances.clone());
                    }
                    DrawTask::DrawIndexed { indices, base_vertex, instances } => {
                        render_pass.draw_indexed(indices.clone(), *base_vertex, instances.clone())
                    }
                }
            }
        }

        self.wgpu.queue.submit(&[encoder.finish()]);

        Ok(())
    }
}

/// Context that's available in any place where rendering/drawing is needed
pub struct GraphicsContext<'a> {
    pub(crate) graphics: &'a mut Graphics,
}
