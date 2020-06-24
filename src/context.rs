use crate::a2d::Graphics2D;
use crate::anyhow::Result;
use crate::winit::event_loop::ControlFlow;
use crate::Color;
use crate::SpriteBatch;
use crate::SpriteSheet;
use crate::TextGrid;
use std::rc::Rc;

/// A reference to an instance of this struct is passed to most methods on
/// Game. This struct allows Game methods to be able to interact
/// with yagl about App related things (e.g. requesting to exit)
pub struct AppContext<'a> {
    pub(crate) control_flow: &'a mut ControlFlow,
    pub(crate) graphics: &'a mut Graphics2D,
}

impl<'a> AppContext<'a> {
    pub fn exit(&mut self) {
        *self.control_flow = ControlFlow::Exit;
    }
}

/// Graphics methods
impl<'a> AppContext<'a> {
    /// Returns the bounds of the visible screen
    ///
    /// By default, the scale is set such that
    /// they correspond to the size of the screen in pixels,
    /// but this can be overriden by explicitly calling set_scale
    ///
    /// When the window is resized, the scale will always be
    /// rescaled to match the default.
    /// To override this behavior, you should implement Game::resize
    ///
    pub fn scale(&self) -> [f32; 2] {
        self.graphics.scale()
    }

    pub fn set_scale(&mut self, scale: [f32; 2]) {
        self.graphics.set_scale(scale)
    }

    /// Creates a new sprite sheet from the bytes of some image
    /// file.
    /// The data is ultimately parsed passing to the
    /// `load_from_memory` function in the `image` crate.
    pub fn new_sheet_from_bytes(&mut self, bytes: &[u8]) -> Result<Rc<SpriteSheet>> {
        Ok(SpriteSheet::from_bytes(self.graphics, bytes)?)
    }

    /// Creates a new sprite sheet from explicitly specified colors
    pub fn new_sheet_from_colors<C, V>(
        &mut self,
        width: u32,
        height: u32,
        colors: V,
    ) -> Result<Rc<SpriteSheet>>
    where
        C: Into<Color>,
        V: IntoIterator<Item = C>,
    {
        Ok(SpriteSheet::from_colors(
            self.graphics,
            width,
            height,
            colors,
        )?)
    }

    /// Creates a new sprite sheet 1 pixel by 1 pixel wide
    /// with the given color
    pub fn new_sheet_from_color<C: Into<Color>>(&mut self, color: C) -> Result<Rc<SpriteSheet>> {
        Ok(SpriteSheet::from_color(self.graphics, color)?)
    }

    /// Creates a new sprite sheet from a Vec<u8> of rgba data
    pub fn new_sheet_from_rgba_bytes(
        &mut self,
        width: u32,
        height: u32,
        bytes: Vec<u8>,
    ) -> Result<Rc<SpriteSheet>> {
        Ok(SpriteSheet::from_rgba_bytes(
            self.graphics,
            width,
            height,
            bytes,
        )?)
    }

    /// Creates a new SpriteBatch from a SpriteSheet
    pub fn new_batch(&mut self, sheet: Rc<SpriteSheet>) -> Result<SpriteBatch> {
        Ok(SpriteBatch::new(sheet))
    }

    pub fn new_batch_from_color<C: Into<Color>>(&mut self, color: C) -> Result<SpriteBatch> {
        let sheet = self.new_sheet_from_color(color)?;
        self.new_batch(sheet)
    }

    /// Create a new text grid from a character width and
    /// [nrows, ncols] pair.
    /// Uses the default courier font bundled with A2D
    pub fn new_text_grid(&mut self, char_width: f32, dim: [u32; 2]) -> Result<TextGrid> {
        Ok(self.graphics.new_text_grid(char_width, dim)?)
    }
}

pub struct RenderContext<'a>
{
    pub(crate) actx: &'a mut AppContext<'a>,
}

impl<'a> RenderContext<'a> {
    pub fn actx(&mut self) -> &mut AppContext<'a> {
        self.actx
    }

    pub fn render(&mut self, batches: &[&SpriteBatch]) {
        self.actx.graphics.render(batches)
    }
}
