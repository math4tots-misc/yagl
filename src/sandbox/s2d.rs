use crate::Game;
use crate::AppContext;
use crate::GraphicsContext;
use crate::TestDrawable;
use crate::Drawable;
use crate::anyhow::Result;

struct SampleGame {
    td: TestDrawable,
}

impl Game for SampleGame {
    fn resize(&mut self, _actx: &mut AppContext, width: u32, height: u32) -> Result<()> {
        println!("Resized: ({}, {})", width, height);
        Ok(())
    }

    fn draw(&self, actx: &mut AppContext, gctx: &mut GraphicsContext) -> Result<()> {
        self.td.draw(actx, gctx)?;
        Ok(())
    }
}

pub fn s2d_main() {
    crate::run(|_actx| Ok(SampleGame {
        td: TestDrawable::new(),
    }));
}
