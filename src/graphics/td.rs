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
use crate::anyhow::Context;
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
            DrawTask::SetVertexBuffer {
                slot: 0,
                buffer: data.vertex_buffer.clone(),
                offset: 0,
                size: 0,
            },
            DrawTask::Draw { vertices: 0..data.num_vertices, instances: 0..1 },
        ])
    }
}

struct Data {
    num_vertices: u32,
    vertex_buffer: Rc<wgpu::Buffer>,
}

impl Data {
    fn new(actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<Self> {
        let device = &gctx.graphics.device;
        let vertex_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(VERTICES),
            wgpu::BufferUsage::VERTEX,
        ).into();
        Ok(Self {
            num_vertices: VERTICES.len() as u32,
            vertex_buffer,
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
    Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    Vertex { position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0] },
    Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];
