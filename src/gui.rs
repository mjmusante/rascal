use bracket_terminal::prelude::*;

pub struct Gui {
    screen: [u8; Gui::ROWS * Gui::COLS],
    cursor_x: usize,
    cursor_y: usize,
    invmode: u8,
}

impl Gui {
    pub const ROWS: usize = 25;
    pub const COLS: usize = 40;

    pub fn new() -> Gui {
        Gui {
            screen: [0u8; Gui::ROWS * Gui::COLS],
            cursor_x: 0,
            cursor_y: 0,
            invmode: 0,
        }
    }

    pub fn draw(&self, ctx: &mut BTerm) {
        for row in 0..Gui::ROWS {
            for col in 0..Gui::COLS {
                let val = self.screen[row * Gui::COLS + col];
                if val < 128 {
                    ctx.set(col, row, RGB::named(GREEN), RGB::named(BLACK), val);
                } else {
                    ctx.set(col, row, RGB::named(BLACK), RGB::named(GREEN), val - 128);
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
                self.invmode = 128;
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
                self.invmode = 0;
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
                self.screen[self.cursor_y * Gui::COLS + self.cursor_x] = ch + self.invmode;
                self.move_right();
            }
            64..=95 => {
                self.screen[self.cursor_y * Gui::COLS + self.cursor_x] = ch - 64 + self.invmode;
                self.move_right();
            }
            _ => {
                panic!("Unhandled character {}", ch);
            }
        }
    }

    pub fn write_string<T: ToString>(&mut self, txt: &T) {
        let mut esc_state = false;
        let mut num = 0;

        for ch in txt.to_string().chars() {
            if esc_state {
                if ch == '}' {
                    self.write_char(num);
                    esc_state = false;
                } else {
                    num = num * 10 + (ch as u8 - b'0');
                }
            } else if ch == '{' {
                esc_state = true;
                num = 0;
            } else {
                self.write_char(ch as u8);
            }
        }
    }

    pub fn write_at<T: ToString>(&mut self, x: usize, y: usize, txt: &T) {
        self.cursor_x = x;
        self.cursor_y = y;
        self.write_string(txt);
    }

    pub fn center<T: ToString>(&mut self, row: usize, txt: &T) {
        let mut len = 0;
        let mut idx = 0;
        let mut in_esc = false;
        let charlist: Vec<char> = txt.to_string().chars().collect();
        while idx < charlist.len() {
            if charlist[idx] == '{' {
                in_esc = true;
            } else if in_esc && charlist[idx] == '}' {
                in_esc = false;
            } else if !in_esc {
                len += 1;
            }
            idx += 1;
        }

        self.write_at((Gui::COLS - len) / 2, row, txt);
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        if x < Gui::COLS && y < Gui::ROWS {
            self.screen[y * Gui::COLS + x] = val;
        }
    }

    // xpos, ypos, width, height
    pub fn map_window(&self) -> (usize, usize, usize, usize) {
        (0, 3, 40, 21)
    }

    pub fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize) {
        for col in x + 1..x + width {
            self.set(col, y, 64);
            self.set(col, y + height, 64);
        }
        for row in y + 1..y + height {
            self.set(x, row, 93);
            self.set(x + width, row, 93);
        }

        self.set(x, y, 112);
        self.set(x + width, y, 110);
        self.set(x, y + height, 109);
        self.set(x + width, y + height, 125);

        for row in y + 1..y + height {
            for col in x + 1..x + width {
                self.set(col, row, 32);
            }
        }
    }
}
