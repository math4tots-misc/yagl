use crate::anyhow::Result;
use crate::anyhow::anyhow;
use crate::winit::{
    window::Window,
};
use gfx_hal::{
    command::{ClearColor, ClearValue},
    format::{Aspects, ChannelType, Format, Swizzle},
    image::{Access, Layout, SubresourceRange, ViewKind},
    pass::{
        Attachment,
        AttachmentLoadOp,
        AttachmentOps,
        AttachmentStoreOp,
        Subpass,
        SubpassDependency,
        SubpassDesc,
    },
    pool::CommandPoolCreateFlags,
    pso::{
        BlendState, ColorBlendDesc, ColorMask, EntryPoint, GraphicsPipelineDesc,
        GraphicsShaderSet,
        PipelineStage, Rasterizer, Rect, Viewport,
    },
    adapter::Adapter,
    queue::Submission,
    Instance,
};


/// gfx-hal logic
pub struct Hal {
}

impl Hal {
    pub unsafe fn new(window: &Window) -> Result<Self> {
        let instance = match gfxb::Instance::create("demo", 1) {
            Ok(i) => i,
            Err(error) =>
                return Err(anyhow!("Failed to create gfx backend: {:?}", error)),
        };
        let surface = instance.create_surface(window)?;
        let adapter = instance.enumerate_adapters().remove(0);
        let num_queues = 1;
        Ok(Self {
        })
    }
}
