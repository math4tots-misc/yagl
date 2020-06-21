use crate::anyhow::{Result, Context};
use crate::wgpu;
use crate::shaders;
use std::rc::Rc;

/// Global graphics stuff for drawing meshes
#[allow(dead_code)]
pub(crate) struct MeshStuff {
    pub(crate) bind_group_layout: Rc<wgpu::BindGroupLayout>,
    pub(crate) render_pipeline: Rc<wgpu::RenderPipeline>,
}

impl MeshStuff {
    pub(super) fn new(wg: &super::wg::Wgpu) -> Result<Self> {
        let device = &wg.device;
        let sc_desc = &wg.sc_desc;

        // TODO: Use mesh shaders instead of the test shaders
        let vs_spirv = wgpu::read_spirv(std::io::Cursor::new(shaders::FIXED_VERT))
            .context("Failed to read Spir-V vertex shader")?;

        let fs_spirv = wgpu::read_spirv(std::io::Cursor::new(shaders::FIXED_FRAG))
            .context("Failed to read Spir-V fragment shader")?;

        let vertex_shader = device.create_shader_module(&vs_spirv);
        let fragment_shader = device.create_shader_module(&fs_spirv);

        let bind_group_layout = Rc::new(device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                    },
                },
            ],
            label: Some("uniform_bind_group_layout"),
        }));

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[
                &bind_group_layout,
            ],
        });

        let render_pipeline = Rc::new(device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vertex_shader,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fragment_shader,
                entry_point: "main",
            }),
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
                vertex_buffers: &[
                    // TestDrawable::vertex_buffer_descriptor(),
                ],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        }));

        Ok(Self {
            bind_group_layout,
            render_pipeline,
        })
    }
}
