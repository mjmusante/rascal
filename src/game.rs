use bracket_terminal::prelude::*;
use specs::prelude::*;

use crate::{
    components::{Item, Name, Position, Renderable, WantsToPickUp},
    gui::Gui,
    interface::InterfaceMode,
    map::Map,
};

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
            let player = ecs.read_resource::<Entity>();
            let positions = ecs.read_storage::<Position>();

            if let Some(p) = positions.get(*player) {
                self.map.draw(gui, p.loc.x as usize, p.loc.y as usize);
                self.redraw = false;
            }
        }

        match self.stage {
            Stage::Init => {
                gui.clear_screen();
                gui.write_string(&"WELCOME!");

                {
                    let player = ecs.read_resource::<Entity>();
                    let positions = ecs.read_storage::<Position>();
                    if let Some(p) = positions.get(*player) {
                        self.map.reveal(p.loc, 3);
                        self.redraw = true;
                    }
                }

                ecs.create_entity()
                    .with(Item {})
                    .with(Name {
                        name: "POTION".to_string(),
                    })
                    .with(Renderable { glyph: 33 })
                    .with(Position {
                        loc: Point { x: 46, y: 26 },
                    })
                    .build();

                self.stage = Stage::Wait;
            }
            Stage::Wait => {}
        }
    }

    pub fn set_player_loc(&mut self, ecs: &mut World, newloc: Point) {
        let player = ecs.write_resource::<Entity>();
        let mut positions = ecs.write_storage::<Position>();

        if let Some(p) = positions.get_mut(*player) {
            p.loc = newloc;
            self.map.reveal(p.loc, 3);
            self.redraw = true;
        }
    }

    pub fn redraw(&mut self) {
        self.redraw = true;
    }

    pub fn handle_key(&mut self, ecs: &mut World, key: VirtualKeyCode) -> Option<InterfaceMode> {
        match key {
            VirtualKeyCode::H => self.try_move(ecs, -1, 0),
            VirtualKeyCode::J => self.try_move(ecs, 0, 1),
            VirtualKeyCode::K => self.try_move(ecs, 0, -1),
            VirtualKeyCode::L => self.try_move(ecs, 1, 0),
            VirtualKeyCode::G => self.want_to_get(ecs),
            VirtualKeyCode::I => {
                return Some(InterfaceMode::ShowInventory);
            }
            VirtualKeyCode::Q => std::process::exit(0),
            _ => {}
        };

        None
    }

    fn want_to_get(&self, ecs: &mut World) {
        let player = ecs.write_resource::<Entity>();
        let positions = ecs.read_storage::<Position>();
        let entities = ecs.entities();

        if let Some(ppos) = positions.get(*player) {
            let items = ecs.read_storage::<Item>();

            for (entity, _, p) in (&entities, &items, &positions).join() {
                if *ppos == *p {
                    let mut pickup = ecs.write_storage::<WantsToPickUp>();
                    pickup
                        .insert(
                            *player,
                            WantsToPickUp {
                                collected_by: *player,
                                entity,
                            },
                        )
                        .expect("Unable to insert want to pickup");
                    break;
                }
            }
        }
    }

    fn try_move(&mut self, ecs: &mut World, delta_x: i32, delta_y: i32) {
        let mut x: Option<Point> = None;

        {
            let player = ecs.read_resource::<Entity>();
            let positions = ecs.read_storage::<Position>();

            if let Some(p) = positions.get(*player) {
                let newx = p.loc.x + delta_x;
                let newy = p.loc.y + delta_y;
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

    pub fn can_see(&self, x: i32, y: i32) -> bool {
        self.map.is_visible(x, y)
    }
}
