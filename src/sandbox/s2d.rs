use crate::DrawTask;
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

    fn draw(&mut self, actx: &mut AppContext, mut gctx: GraphicsContext) -> Result<Vec<DrawTask>> {
        self.td.draw(actx, &mut gctx)
    }
}

pub fn s2d_main() {
    crate::run(|_actx| Ok(SampleGame {
        td: TestDrawable::new(),
    }));
}
