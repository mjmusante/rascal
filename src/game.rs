use bracket_terminal::prelude::*;

use crate::{gui::Gui, map::Map};

pub struct Game {
    stage: Stage,
    player: Point,
    map: Map,
    redraw: bool,
}

enum Stage {
    Init,
    Wait,
}

impl Game {
    pub fn new() -> Game {
        Game {
            stage: Stage::Init,
            player: Point { x: 0, y: 0 },
            map: Map::new(128, 128),
            redraw: false,
        }
    }

    pub fn run(&mut self, gui: &mut Gui) {
        match self.stage {
            Stage::Init => {
                gui.clear_screen();
                self.player = self.map.get_upstairs();

                self.map.reveal(self.player, 3);
                self.map
                    .draw(gui, self.player.x as usize, self.player.y as usize);

                self.map.put_char(gui, self.player, 0);

                self.stage = Stage::Wait;
            }
            Stage::Wait => {
                if self.redraw {
                    self.map
                        .draw(gui, self.player.x as usize, self.player.y as usize);
                    self.redraw = false;
                    self.map.put_char(gui, self.player, 0);
                }
            }
        }
    }

    pub fn handle_key(&mut self, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::H => self.try_move(-1, 0),
            VirtualKeyCode::J => self.try_move(0, 1),
            VirtualKeyCode::K => self.try_move(0, -1),
            VirtualKeyCode::L => self.try_move(1, 0),
            VirtualKeyCode::Q => std::process::exit(0),
            _ => {}
        };
    }

    fn try_move(&mut self, delta_x: i32, delta_y: i32) {
        let newx = self.player.x + delta_x;
        let newy = self.player.y + delta_y;
        if self.map.can_go(newx, newy) {
            self.player = Point { x: newx, y: newy };
            self.map.reveal(self.player, 3);
            self.redraw = true;
        }
    }
}
