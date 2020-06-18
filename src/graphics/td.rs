#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::Drawable;
use crate::Globals;
use crate::AppContext;
use crate::GraphicsContext;
use crate::anyhow::Result;
use crate::shaders;
use wgpu::RenderPass;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;


pub struct TestDrawable {
    data: RefCell<Option<Data>>,
}

impl TestDrawable {
    pub fn new() -> TestDrawable { Self {
        data: RefCell::new(None),
    } }

    fn data(&self, actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<Ref<Data>> {
        if self.data.borrow().is_none() {
            *self.data.borrow_mut() = Some(Data::new(actx, gctx)?);
        }
        Ok(Ref::map(self.data.borrow(), |d| d.as_ref().unwrap()))
    }
}

impl Drawable for TestDrawable {
    fn draw(&self, actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<()> {
        Ok(())
    }
}

struct Data {
    vertex_shader: wgpu::ShaderModule,
}

impl Data {
    fn new(actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<Data> {
        let vs_spirv = wgpu::read_spirv(std::io::Cursor::new(shaders::fixed))?;

        let device = &gctx.graphics.device;
        let sc_desc = &gctx.graphics.sc_desc;

        let vertex_shader = device.create_shader_module(&vs_spirv);
        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vertex_shader,
                entry_point: "main",
            },
            fragment_stage: None,
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            color_states: &[
                wgpu::ColorStateDescriptor {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendDescriptor::REPLACE,
                    alpha_blend: wgpu::BlendDescriptor::REPLACE,
                    write_mask: wgpu::ColorWrite::ALL,
                },
            ],
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        Ok(Data {
            vertex_shader,
        })
    }
}
