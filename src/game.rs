use crate::gui::*;

pub struct Game {
    stage: Stage,
}

enum Stage {
    Init,
    Wait,
}

impl Game {
    pub fn new() -> Game {
        Game { stage: Stage::Init, }
    }

    pub fn run(&mut self, gui: &mut Gui) {
        match self.stage {
            Stage::Init => { gui.clear_screen(); self.stage = Stage::Wait; }
            Stage::Wait => { }
        }
    }
}
