use bracket_terminal::prelude::*;

pub struct Gui {
    screen: [u8; Gui::ROWS * Gui::COLS],
    cursor_x: usize,
    cursor_y: usize,
    invmode: bool,
}

impl Gui {
    pub const ROWS: usize = 25;
    pub const COLS: usize = 40;

    pub fn new() -> Gui {
        Gui {
            screen: [0u8; Gui::ROWS * Gui::COLS],
            cursor_x: 0,
            cursor_y: 0,
            invmode: false,
        }
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        for row in 0..Gui::ROWS {
            for col in 0..Gui::COLS {
                let val = self.screen[row * Gui::COLS + col];
                if val < 128 {
                    ctx.set(col, row, RGB::named(GREEN), RGB::named(BLACK), val);
                } else {
                    ctx.set(col, row, RGB::named(GREEN), RGB::named(BLACK), val - 128);
                }
            }
        }
    }

    pub fn clear_screen(&mut self) {
        self.screen = [32u8; Gui::ROWS * Gui::COLS];
        self.cursor_x = 0;
        self.cursor_y = 0;
    }

    pub fn next_line(&mut self) {
        if self.cursor_y + 1 >= Gui::ROWS {
            for i in Gui::ROWS..self.screen.len() {
                self.screen[i - Gui::ROWS] = self.screen[i];
            }
        } else {
            self.cursor_y += 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor_x + 1 >= Gui::COLS {
            self.cursor_x = 0;
            self.next_line();
        } else {
            self.cursor_x += 1;
        }
    }

    fn write_char(&mut self, ch: u8) {
        match ch {
            13 => {
                self.cursor_x = 0;
                self.next_line();
            }
            17 => {
                self.next_line();
            }
            18 => {
                self.invmode = true;
            }
            19 => {
                self.cursor_x = 0;
                self.cursor_y = 0;
            }
            29 => self.move_right(),
            145 => {
                if self.cursor_y > 0 {
                    self.cursor_y -= 1;
                }
            }
            146 => {
                self.invmode = false;
            }
            147 => self.clear_screen(),
            157 => {
                if self.cursor_x > 0 {
                    self.cursor_x -= 1;
                } else if self.cursor_y > 1 {
                    self.cursor_x = Gui::COLS - 1;
                    self.cursor_y -= 1;
                }
            }
            32..=63 => {
                self.screen[self.cursor_y * Gui::COLS + self.cursor_x] = ch;
                self.move_right();
            }
            64..=95 => {
                self.screen[self.cursor_y * Gui::COLS + self.cursor_x] = ch - 64;
                self.move_right();
            }
            _ => {
                panic!("Unhandled character {}", ch);
            }
        }
    }

    pub fn write_string<T: ToString>(&mut self, txt: &T) {
        for ch in txt.to_string().chars() {
            self.write_char(ch as u8);
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        if x < Gui::COLS && y < Gui::ROWS {
            self.screen[y * Gui::COLS + x] = val;
        }
    }
}
