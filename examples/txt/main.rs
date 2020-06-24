#![allow(dead_code)]
#![allow(unused_variables)]
extern crate anyhow;
extern crate yagl;

use anyhow::Result;
use yagl::AppContext;
use yagl::DeviceId;
use yagl::Instance;
use yagl::Key;
use yagl::SpriteBatch;
use yagl::TextGrid;

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
        if ch >= '!' && (ch as u32) < 127 {
            self.text.write_ch(actx, ch)?;
        }
        Ok(())
    }

    fn key_pressed(&mut self, actx: &mut AppContext, _: DeviceId, key: Key) -> Result<()> {
        match key {
            Key::Escape => {
                actx.exit();
            }
            Key::Backspace => {
                self.text.backspace(actx)?;
            }
            Key::Right => {
                self.text.move_right(actx)?;
            }
            Key::Left => {
                self.text.move_left(actx)?;
            }
            Key::Up => {
                self.text.move_up(actx)?;
            }
            Key::Down => {
                self.text.move_down(actx)?;
            }
            Key::Return => {
                self.text.enter(actx)?;
            }
            _ => {}
        }
        Ok(())
    }
}

fn main() {
    yagl::run(|actx| {
        let text = Text::new(actx, "Hello world").unwrap();
        println!("scale = {:?}", actx.scale());
        Ok(Game { text })
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
        self.sprite
            .get_mut(0)
            .set_color_factor([1.0, 1.0, 1.0, alpha]);
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
        let mut cursor = Cursor {
            pos: [0, 0],
            sprite: actx.new_batch_from_color([1.0, 1.0, 1.0])?,
        };
        cursor.sprite.add(Instance::builder().build());
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

    fn write_ch(&mut self, actx: &mut AppContext, ch: char) -> Result<()> {
        let [row, col] = self.cursor.pos;

        if (row as usize) < self.lines.len() {
            self.lines[row as usize].insert(col as usize, ch);
        } else {
            while row as usize >= self.lines.len() {
                self.lines.push(Vec::new());
            }
            self.lines.last_mut().unwrap().push(ch);
            self.move_cursor(actx, [row, 0])?;
        }

        self.grid.write_ch(self.cursor.pos, ch);
        self.move_cursor(actx, self.right_pos(self.cursor.pos))
    }

    fn backspace(&mut self, actx: &mut AppContext) -> Result<()> {
        if self.cursor.pos[1] == 0 {
            let [row, col] = self.cursor.pos;
            if row > 0 {
                let line = self.lines.remove(row as usize);
                let new_col = self.lines[row as usize - 1].len() as u32;
                self.lines[row as usize - 1].extend(line);
                self.move_cursor(actx, [row - 1, new_col])?;
            }
        } else {
            self.move_left(actx)?;

            let [row, start_col] = self.cursor.pos;
            self.lines[row as usize].remove(start_col as usize);

            for (col, ch) in self.lines[row as usize].iter().enumerate() {
                self.grid.write_ch([row, col as u32], *ch);
            }
            self.grid
                .write_ch([row, self.lines[row as usize].len() as u32], ' ');
            self.move_cursor(actx, [row, start_col])?;
        }
        Ok(())
    }

    fn move_down(&mut self, actx: &mut AppContext) -> Result<()> {
        let new_pos = self.down_pos(self.cursor.pos);
        self.move_cursor(actx, new_pos)?;
        Ok(())
    }

    fn move_up(&mut self, actx: &mut AppContext) -> Result<()> {
        let new_pos = self.up_pos(self.cursor.pos);
        self.move_cursor(actx, new_pos)?;
        Ok(())
    }

    fn move_right(&mut self, actx: &mut AppContext) -> Result<()> {
        let new_pos = self.right_pos(self.cursor.pos);
        self.move_cursor(actx, new_pos)?;
        Ok(())
    }

    fn move_left(&mut self, actx: &mut AppContext) -> Result<()> {
        let new_pos = self.left_pos(self.cursor.pos);
        self.move_cursor(actx, new_pos)?;
        Ok(())
    }

    fn enter(&mut self, actx: &mut AppContext) -> Result<()> {
        let [row, col] = self.cursor.pos;
        let rest_of_line = self.lines[row as usize].split_off(col as usize);
        self.lines.insert(row as usize + 1, rest_of_line);
        self.move_cursor(actx, [row + 1, 0])?;
        Ok(())
    }

    fn up_pos(&self, pos: [u32; 2]) -> [u32; 2] {
        let [row, col] = pos;
        if row == 0 {
            [0, 0]
        } else {
            [
                row - 1,
                std::cmp::min(col, self.lines[row as usize].len() as u32),
            ]
        }
    }

    fn down_pos(&self, pos: [u32; 2]) -> [u32; 2] {
        let [row, col] = pos;
        if row as usize >= self.lines.len() - 1 {
            [
                self.lines.len() as u32 - 1,
                self.lines.last().unwrap().len() as u32,
            ]
        } else {
            [
                row - 1,
                std::cmp::min(col, self.lines[row as usize].len() as u32),
            ]
        }
    }

    fn right_pos(&self, pos: [u32; 2]) -> [u32; 2] {
        let nrows = self.lines.len() as u32;
        let [row, col] = pos;
        if row >= nrows {
            [
                nrows - 1,
                self.lines.last().map(|v| v.len()).unwrap_or(0) as u32,
            ]
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
