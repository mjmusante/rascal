use bracket_terminal::prelude::*;

pub struct Gui {
    screen: [u8; Gui::ROWS * Gui::COLS],
}

impl Gui {
    const ROWS : usize = 25;
    const COLS : usize = 40;

    pub fn new() -> Gui {
        Gui { screen: [0u8; Gui::ROWS * Gui::COLS] }
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
    }
}
