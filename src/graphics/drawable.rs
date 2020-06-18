use crate::anyhow::Result;
use crate::AppContext;
use crate::GraphicsContext;

pub trait Drawable {
    fn draw(&self, actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<()>;
}
