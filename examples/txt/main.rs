extern crate yagl;
extern crate anyhow;

use anyhow::Result;
use yagl::SpriteBatch;
use yagl::TextGrid;
use yagl::AppContext;

struct Game {
    text: Text,
}

impl yagl::Game for Game {
    fn update(&mut self, actx: &mut AppContext) -> Result<()> {
        self.text.update();
        Ok(())
    }

    fn render(&mut self, actx: &mut AppContext) -> Result<Vec<&SpriteBatch>> {
        Ok(self.text.batches())
    }

    fn ch(&mut self, actx: &mut AppContext, ch: char) -> Result<()> {
        self.text.write_ch(actx, ch);
        Ok(())
    }
}

fn main() {
    yagl::run(|actx| {
        let text = Text::new(actx, "Hello world").unwrap();
        println!("scale = {:?}", actx.scale());
        Ok(Game {
            text,
        })
    })
}

struct Cursor {
    /// row and column
    pos: [u32; 2],
    sprite: SpriteBatch,
}

impl Cursor {
    fn move_to(&mut self, actx: &mut AppContext, pos: [u32; 2], grid: &TextGrid) -> Result<()> {
        if self.pos != pos {
            self.pos = pos;
            let rect = grid.rect_for_coord(pos);
            self.sprite.get_mut(0).set_dest(rect);
        }
        Ok(())
    }

    fn set_visible(&mut self, visible: bool) {
        let alpha = if visible { 1.0 } else { 0.0 };
        self.sprite.get_mut(0).set_color_factor([1.0, 1.0, 1.0, alpha]);
    }
}

struct Text {
    start: std::time::SystemTime,
    cursor: Cursor,
    grid: TextGrid,
    lines: Vec<Vec<char>>,
}

impl Text {
    fn new(actx: &mut AppContext, s: &str) -> Result<Self> {
        let grid = actx.new_text_grid(24.0, [120, 80])?;
        let lines = s.lines().map(|line| line.chars().collect()).collect();
        let cursor = Cursor {
            pos: [0, 0],
            sprite: actx.new_batch_from_color([1.0, 1.0, 1.0])?,
        };
        let mut text = Text {
            start: std::time::SystemTime::now(),
            cursor,
            grid,
            lines,
        };
        text.set_dimensions(actx, 24.0, [120, 80]);
        text.move_cursor(actx, [0, 0])?;
        Ok(text)
    }

    fn update(&mut self) {
        if self.start.elapsed().unwrap().as_secs_f64().fract() < 0.5 {
            self.cursor.set_visible(false);
        } else {
            self.cursor.set_visible(true);
        }
    }

    fn batches(&self) -> Vec<&SpriteBatch> {
        vec![self.grid.batch(), &self.cursor.sprite]
    }

    fn move_cursor(&mut self, actx: &mut AppContext, pos: [u32; 2]) -> Result<()> {
        self.cursor.move_to(actx, pos, &self.grid)
    }

    fn write_ch(&mut self, actx: &mut AppContext, ch: char) {
        let [row, col] = self.cursor.pos;

        if (row as usize) < self.lines.len() {
            self.lines[row as usize].insert(col as usize, ch);
        } else {
            while row as usize >= self.lines.len() {
                self.lines.push(Vec::new());
            }
            self.lines.last_mut().unwrap().push(ch);
            self.move_cursor(actx, [row, 0]);
        }

        self.grid.write_ch(self.cursor.pos, ch);
        self.move_cursor(actx, self.right_pos(self.cursor.pos));
    }

    fn right_pos(&self, pos: [u32; 2]) -> [u32; 2] {
        let nrows = self.lines.len() as u32;
        let [row, col] = pos;
        if row >= nrows {
            [nrows - 1, self.lines.last().map(|v| v.len()).unwrap_or(0) as u32]
        } else if col as usize >= self.lines[row as usize].len() {
            [std::cmp::min(row + 1, nrows - 1), 0]
        } else {
            [row, col + 1]
        }
    }

    fn left_pos(&self, pos: [u32; 2]) -> [u32; 2] {
        let [row, col] = pos;
        if col == 0 {
            [if row > 0 { row - 1 } else { 0 }, 0]
        } else {
            [row, col - 1]
        }
    }

    fn set_dimensions(&mut self, actx: &mut AppContext, char_width: f32, nrows_ncols: [u32; 2]) {
        self.grid = actx.new_text_grid(char_width, nrows_ncols).unwrap();
        for (row, line) in self.lines.iter().enumerate() {
            for (col, ch) in line.iter().enumerate() {
                self.grid.write_ch([row as u32, col as u32], *ch);
            }
        }
    }
}
