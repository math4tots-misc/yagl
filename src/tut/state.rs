use crate::wgpu;
use crate::bytemuck;
use crate::shaderc;
use crate::winit::{
    event::{
        WindowEvent,
    },
    window::{
        Window,
    },
};
use super::VERTICES;
use super::INDICES;
use super::Vertex;
use super::Texture;
use super::Camera;
use super::CameraController;
use super::Uniforms;

#[allow(dead_code)]
pub(super) struct State {
    pub(super) surface: wgpu::Surface,
    pub(super) adapter: wgpu::Adapter,

    pub(super) device: wgpu::Device,
    pub(super) queue: wgpu::Queue,
    pub(super) sc_desc: wgpu::SwapChainDescriptor,
    pub(super) swap_chain: wgpu::SwapChain,

    pub(super) diffuse_texture: Texture,
    pub(super) diffuse_bind_group: wgpu::BindGroup,

    pub(super) camera: Camera,
    pub(super) camera_controller: CameraController,
    pub(super) uniforms: Uniforms,
    pub(super) uniform_buffer: wgpu::Buffer,
    pub(super) uniform_bind_group: wgpu::BindGroup,

    pub(super) render_pipeline: wgpu::RenderPipeline,
    pub(super) vertex_buffer: wgpu::Buffer,
    pub(super) index_buffer: wgpu::Buffer,
    pub(super) num_vertices: u32,
    pub(super) num_indices: u32,
    pub(super) size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    fn compile_shaders(
        device: &wgpu::Device,
        sc_desc: &wgpu::SwapChainDescriptor,
        texture_bind_group_layout: &wgpu::BindGroupLayout,
        uniform_bind_group_layout: &wgpu::BindGroupLayout,
    ) -> wgpu::RenderPipeline {
        let vs_src = include_str!("shader.vert");
        let fs_src = include_str!("shader.frag");

        let mut compiler = shaderc::Compiler::new().unwrap();
        let vs_spirv = compiler.compile_into_spirv(
            vs_src,
            shaderc::ShaderKind::Vertex,
            "shader.vert",
            "main",
            None,
        ).unwrap();
        let fs_spirv = compiler.compile_into_spirv(
            fs_src,
            shaderc::ShaderKind::Fragment,
            "shader.frag",
            "main",
            None,
        ).unwrap();
        let vs_data = wgpu::read_spirv(std::io::Cursor::new(vs_spirv.as_binary_u8())).unwrap();
        let fs_data = wgpu::read_spirv(std::io::Cursor::new(fs_spirv.as_binary_u8())).unwrap();

        let vs_module = device.create_shader_module(&vs_data);
        let fs_module = device.create_shader_module(&fs_data);

        let render_pipeline_layout = device.create_pipeline_layout(
            &wgpu::PipelineLayoutDescriptor {
                bind_group_layouts: &[
                    texture_bind_group_layout,
                    uniform_bind_group_layout,
                ],
            }
        );

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &render_pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
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
                    Vertex::desc(),
                ],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        render_pipeline
    }

    fn new_vertex_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_with_data(
            bytemuck::cast_slice(VERTICES),
            wgpu::BufferUsage::VERTEX,
        )
    }

    fn new_index_buffer(device: &wgpu::Device) -> wgpu::Buffer {
        device.create_buffer_with_data(
            bytemuck::cast_slice(INDICES),
            wgpu::BufferUsage::INDEX,
        )
    }

    fn load_texture(device: &wgpu::Device, queue: &mut wgpu::Queue) -> (
        Texture,
        wgpu::BindGroup,
        wgpu::BindGroupLayout,
    ) {
        let diffuse_bytes = include_bytes!("happy-tree.png");
        let (diffuse_texture, cmd_buffer) = Texture::from_bytes(device, diffuse_bytes).unwrap();
        queue.submit(&[cmd_buffer]);

        let texture_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        dimension: wgpu::TextureViewDimension::D2,
                        component_type: wgpu::TextureComponentType::Uint,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        comparison: false,
                    }
                },
            ],
            label: Some("texture_bind_group_layout"),
        });

        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        (
            diffuse_texture,
            diffuse_bind_group,
            texture_bind_group_layout,
        )
    }

    pub(super) async fn new(window: &Window) -> Self {
        let num_vertices = VERTICES.len() as u32;
        let num_indices = INDICES.len() as u32;
        let size = window.inner_size();
        let surface = wgpu::Surface::create(window);
        let adapter = wgpu::Adapter::request(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::Default,
                compatible_surface: Some(&surface),
            },
            wgpu::BackendBit::PRIMARY,
        ).await.unwrap();
        let (device, mut queue) = adapter.request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: Default::default(),
        }).await;
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };
        let swap_chain = device.create_swap_chain(&surface, &sc_desc);

        let (
            diffuse_texture,
            diffuse_bind_group,
            texture_bind_group_layout,
         ) = Self::load_texture(&device, &mut queue);

         let camera = Camera {
            // position the camera one unit up and 2 units back
            // +z is out of the screen
            eye: (0.0, 1.0, 2.0).into(),
            // have it look at the origin
            target: (0.0, 0.0, 0.0).into(),
            // which way is "up"
            up: cgmath::Vector3::unit_y(),
            aspect: sc_desc.width as f32 / sc_desc.height as f32,
            fovy: 45.0,
            znear: 0.1,
            zfar: 100.0,
        };

        let camera_controller = CameraController::new(0.2);

        let mut uniforms = Uniforms::new();
        uniforms.update_view_proj(&camera);

        let uniform_buffer = device.create_buffer_with_data(
            bytemuck::cast_slice(&[uniforms]),
            wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        );

        let uniform_bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                    },
                }
            ],
            label: Some("uniform_bind_group_layout"),
        });

        let uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &uniform_bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::Buffer {
                        buffer: &uniform_buffer,
                        // FYI: you can share a single buffer between bindings.
                        range: 0..std::mem::size_of_val(&uniforms) as wgpu::BufferAddress,
                    }
                }
            ],
            label: Some("uniform_bind_group"),
        });

        let render_pipeline = Self::compile_shaders(
            &device,
            &sc_desc,
            &texture_bind_group_layout,
            &uniform_bind_group_layout,
        );
        let vertex_buffer = Self::new_vertex_buffer(&device);
        let index_buffer = Self::new_index_buffer(&device);
        Self {
            surface,
            adapter,
            device,
            queue,
            sc_desc,
            swap_chain,
            diffuse_texture,
            diffuse_bind_group,
            camera,
            camera_controller,
            uniforms,
            uniform_buffer,
            uniform_bind_group,
            render_pipeline,
            vertex_buffer,
            index_buffer,
            num_vertices,
            num_indices,
            size,
        }
    }

    pub(super) fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        self.size = new_size;
        self.sc_desc.width = new_size.width;
        self.sc_desc.height = new_size.height;
        self.swap_chain = self.device.create_swap_chain(&self.surface, &self.sc_desc);
    }

    // input() won't deal with GPU code, so it can be synchronous
    // return true if the event is handled
    pub(super) fn input(&mut self, event: &WindowEvent) -> bool {
        self.camera_controller.process_events(event)
    }

    pub(super) fn update(&mut self) {
        self.camera_controller.update_camera(&mut self.camera);
        self.uniforms.update_view_proj(&self.camera);

        // Copy operation's are performed on the gpu, so we'll need
        // a CommandEncoder for that
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("update encoder"),
        });

        let staging_buffer = self.device.create_buffer_with_data(
            bytemuck::cast_slice(&[self.uniforms]),
            wgpu::BufferUsage::COPY_SRC,
        );

        encoder.copy_buffer_to_buffer(&staging_buffer, 0, &self.uniform_buffer, 0, std::mem::size_of::<Uniforms>() as wgpu::BufferAddress);

        // We need to remember to submit our CommandEncoder's output
        // otherwise we won't see any change.
        self.queue.submit(&[encoder.finish()]);
    }

    pub(super) fn render(&mut self) {
        let frame = self.swap_chain.get_next_texture().expect("Timeout getting texture");
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
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
                    }
                ],
                depth_stencil_attachment: None,
            });
            render_pass.set_pipeline(&self.render_pipeline);
            render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);
            render_pass.set_bind_group(1, &self.uniform_bind_group, &[]);
            render_pass.set_vertex_buffer(0, &self.vertex_buffer, 0, 0);
            render_pass.set_index_buffer(&self.index_buffer, 0, 0);
            render_pass.draw_indexed(0..self.num_indices, 0, 0..1);
        }
        self.queue.submit(&[
            encoder.finish(),
        ]);
    }
}
