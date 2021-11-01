use bracket_terminal::prelude::*;

pub struct Interface;

impl GameState for Interface {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.set(3, 3, RGB::named(GREEN), RGB::named(BLACK), 7);
    }
}

impl Interface {
    pub fn new() -> Interface {
        Interface {}
    }
}
