use crate::anyhow::Result;
use crate::AppContext;
use crate::GraphicsContext;
use crate::wgpu;
use std::rc::Rc;
use std::ops::Range;

pub trait Drawable {
    fn draw(&self, actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<Vec<DrawTask>>;
}

// Returning Vec<DrawTask> instead of calling methods on render_pass
// has the benefit that
//    * The set_pipeline method on RenderPass instances require that
//      the RenderPipeline instance live at least as long as the render pass
//      itself. If we pass a &mut RenderPass to the draw method, it's difficult
//      for the Drawable to create any pipelines that it can prove to Rust
//      will outlive the RenderPass.
//      In fact, even if we do go down that route, the signature on draw
//      will need to be more complex to ensure that 'self' outlives
//      the RenderPass object itself. Furthermore, it makes lazy caching
//      much harder, since it's difficult to borrow from a RefCell in a way
//      that would ensure the borrow will outlive the RenderPass
//      (even though RenderPass is shortlived in Graphics::render, from inside
//      Drawable::draw, the only information we have is that the RenderPass outlives
//      the draw call itself.)
//
// Using DrawTask in this way allows us to sidestep the above issue by
// returning a value that the caller can explicitly extend the lifetime of.
//
// The downside is that this is a bit more restrictive, and potentially is
// a bit slower do to the switches and Rc's
//
#[derive(Debug)]
pub enum DrawTask {

    /// Calls wgpu::RenderPass::set_pipeline
    ///
    SetPipeline(Rc<wgpu::RenderPipeline>),

    /// Calls wgpu::RenderPass::draw
    /// Draws primitives from the active vertex buffer(s).
    /// Active vertex buffers can be set with SetVertexBuffers
    Draw {
        // the range of vertex indices to draw
        // this is what sets the 'gl_VertexIndex' in the vertex shaders
        vertices: Range<u32>,

        // the range of instance indices to draw
        // this is what sets the 'gl_InstanceIndex' in the vertex shaders
        instances: Range<u32>,
    },
}
