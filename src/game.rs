use bracket_terminal::prelude::*;
use specs::prelude::*;

use crate::{components::{Item, Renderable}, gui::Gui, map::Map};

pub struct Game {
    stage: Stage,
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
            map: Map::new(50, 30),
            redraw: false,
        }
    }

    pub fn run(&mut self, gui: &mut Gui, ecs: &mut World) {
        if self.redraw {
            let player = ecs.write_resource::<Entity>();
            let mut renderables = ecs.write_storage::<Renderable>();

            if let Some(r) = renderables.get_mut(*player) {
                self.map.draw(gui, r.loc.x as usize, r.loc.y as usize);
                self.redraw = false;
            }
        }

        match self.stage {
            Stage::Init => {
                gui.clear_screen();
                gui.write_string(&"WELCOME!");

                ecs.create_entity()
                    .with(Item{})
                    .with(Renderable { loc: Point { x: 46, y: 26 }, glyph: 33 })
                    .build();

                self.stage = Stage::Wait;
            }
            Stage::Wait => {}
        }
    }

    pub fn set_player_loc(&mut self, ecs: &mut World, newloc: Point) {
        let player = ecs.write_resource::<Entity>();
        let mut renderables = ecs.write_storage::<Renderable>();

        if let Some(r) = renderables.get_mut(*player) {
            r.loc = newloc;
            self.map.reveal(r.loc, 3);
            self.redraw = true;
        }
    }

    pub fn handle_key(&mut self, ecs: &mut World, key: VirtualKeyCode) {
        match key {
            VirtualKeyCode::H => self.try_move(ecs, -1, 0),
            VirtualKeyCode::J => self.try_move(ecs, 0, 1),
            VirtualKeyCode::K => self.try_move(ecs, 0, -1),
            VirtualKeyCode::L => self.try_move(ecs, 1, 0),
            VirtualKeyCode::Q => std::process::exit(0),
            _ => {}
        };
    }

    fn try_move(&mut self, ecs: &mut World, delta_x: i32, delta_y: i32) {
        let mut x: Option<Point> = None;

        {
            let player = ecs.read_resource::<Entity>();
            let renderables = ecs.read_storage::<Renderable>();

            if let Some(r) = renderables.get(*player) {
                let newx = r.loc.x + delta_x;
                let newy = r.loc.y + delta_y;
                if self.map.can_go(newx, newy) {
                    x = Some(Point { x: newx, y: newy });
                }
            }
        }

        if let Some(newloc) = x {
            self.set_player_loc(ecs, newloc);
        }
    }

    pub fn initial_loc(&self) -> Point {
        self.map.get_upstairs()
    }

    pub fn map_offset(&self) -> (i32, i32) {
        self.map.get_offset()
    }
}
