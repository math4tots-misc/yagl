#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
use crate::Drawable;
use crate::Globals;
use crate::AppContext;
use crate::DrawTask;
use crate::GraphicsContext;
use crate::anyhow::Result;
use crate::bytemuck;
use crate::shaders;
use crate::cgmath;
use crate::anyhow::Context;
use cgmath::SquareMatrix;
use wgpu::RenderPass;
use std::rc::Rc;
use std::cell::RefCell;
use std::cell::Ref;


pub struct TestDrawable {
    rotation: f32,
    data: RefCell<Option<Data>>,
}

impl TestDrawable {
    pub fn new() -> TestDrawable { Self {
        rotation: 0.0,
        data: RefCell::new(None),
    } }

    fn data(&self, actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<Ref<Data>> {
        if self.data.borrow().is_none() {
            *self.data.borrow_mut() = Some(Data::new(actx, gctx)?);
        }
        if self.data.borrow().as_ref().unwrap().dirty_uniforms {
            let device = &gctx.graphics.wgpu.device;
            let mut borrow = self.data.borrow_mut();
            let data = borrow.as_mut().unwrap();

            let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Update encoder"),
            });

            let staging_buffer = device.create_buffer_with_data(
                bytemuck::cast_slice(&[data.uniforms]),
                wgpu::BufferUsage::COPY_SRC,
            );
            data.dirty_uniforms = false;

            encoder.copy_buffer_to_buffer(
                &staging_buffer,
                0,
                &data.uniform_buffer,
                0,
                std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
            );

            gctx.graphics.wgpu.queue.submit(&[encoder.finish()]);
        }
        Ok(Ref::map(self.data.borrow(), |d| d.as_ref().unwrap()))
    }

    pub(crate) fn vertex_buffer_descriptor<'a>() -> wgpu::VertexBufferDescriptor<'a> {
        use std::mem;
        wgpu::VertexBufferDescriptor {
            stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::InputStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttributeDescriptor {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float3,
                },
                wgpu::VertexAttributeDescriptor {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float3,
                }
            ]
        }
    }
}

impl Drawable for TestDrawable {
    fn draw(&self, actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<Vec<DrawTask>> {
        let data = self.data(actx, gctx)?;
        let ggl = &actx.globals.graphics;
        Ok(vec![
            DrawTask::SetPipeline(ggl.test.render_pipeline.clone()),
            DrawTask::SetBindGroup {
                index: 0,
                bind_group: data.uniform_bind_group.clone(),
                offsets: Vec::new(),
            },
            DrawTask::SetVertexBuffer {
                slot: 0,
                buffer: data.vertex_buffer.clone(),
                offset: 0,
                size: 0,
            },
            DrawTask::SetIndexBuffer {
                buffer: data.index_buffer.clone(),
                offset: 0,
                size: 0,
            },
            DrawTask::DrawIndexed {
                indices: 0..data.num_indices,
                base_vertex: 0,
                instances: 0..1,
            },
        ])
    }
}

struct Data {
    num_vertices: u32,
    vertex_buffer: Rc<wgpu::Buffer>,
    num_indices: u32,
    index_buffer: Rc<wgpu::Buffer>,

    // For passing the translation and rotation matrix
    dirty_uniforms: bool,
    uniforms: Uniforms,
    uniform_buffer: Rc<wgpu::Buffer>,
    uniform_bind_group: Rc<wgpu::BindGroup>,
}

impl Data {
    fn new(actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<Self> {
        let device = &gctx.graphics.wgpu.device;
        let vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(VERTICES),
            wgpu::BufferUsage::VERTEX,
        ).into();
        let index_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(INDICES),
            wgpu::BufferUsage::INDEX,
        ).into();
        let uniforms = Uniforms::new();
        let uniform_buffer = Rc::new(device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        ));
        let uniform_bind_group = Self::uniform_bind_group(
            &actx.globals,
            &gctx.graphics.wgpu.device,
            &uniform_buffer,
        ).into();
        Ok(Self {
            num_vertices: VERTICES.len() as u32,
            vertex_buffer,
            num_indices: INDICES.len() as u32,
            index_buffer,
            dirty_uniforms: false,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
        })
    }

    fn uniform_bind_group(globals: &Globals, device: &wgpu::Device, uniform_buffer: &wgpu::Buffer) -> wgpu::BindGroup {
        let layout = &globals.graphics.test.bind_group_layout;
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: uniform_buffer,
                        range: 0..std::mem::size_of::<Uniforms>() as wgpu::BufferAddress,
                    },
                },
            ],
            label: Some("uniform_bind_group"),
        })
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
struct Vertex {
    position: [f32; 3],
    color: [f32; 3],
}
unsafe impl bytemuck::Pod for Vertex {}
unsafe impl bytemuck::Zeroable for Vertex {}

const VERTICES: &[Vertex] = &[
    Vertex { position: [-0.0868241, 0.49240386, 0.0], color: [0.5, 0.0, 0.5] }, // A
    Vertex { position: [-0.49513406, 0.06958647, 0.0], color: [0.5, 0.0, 0.5] }, // B
    Vertex { position: [-0.21918549, -0.44939706, 0.0], color: [0.5, 0.0, 0.5] }, // C
    Vertex { position: [0.35966998, -0.3473291, 0.0], color: [0.5, 0.0, 0.5] }, // D
    Vertex { position: [0.44147372, 0.2347359, 0.0],color: [0.5, 0.0, 0.5] }, // E
];

const INDICES: &[u16] = &[
    0, 1, 4,
    1, 2, 4,
    2, 3, 4,
];

#[repr(C)]
#[derive(Debug, Clone, Copy)]
struct Uniforms {
    transformation_matrix: cgmath::Matrix4<f32>,
}

impl Uniforms {
    fn new() -> Self {
        Self {
            transformation_matrix: cgmath::Matrix4::identity(),
        }
    }
}
unsafe impl bytemuck::Pod for Uniforms {}
unsafe impl bytemuck::Zeroable for Uniforms {}
