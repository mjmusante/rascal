use bracket_terminal::prelude::*;
use specs::prelude::*;

use crate::components::*;

use crate::game::*;
use crate::gui::*;
use crate::items::{show_inventory, ItemPickupSystem, MenuResult};

pub struct Interface {
    gui: Gui,
    game: Game,
    ecs: World,
    mode: InterfaceMode,
}

pub enum InterfaceMode {
    Greeting,
    Game,
    ShowInventory,
}

impl GameState for Interface {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match self.mode {
            InterfaceMode::ShowInventory => {
                if show_inventory(&self.ecs, &mut self.gui, ctx.key) == MenuResult::Cancel {
                    self.mode = InterfaceMode::Game;
                    self.game.redraw();
                }
            }
            InterfaceMode::Greeting => {
                if let Some(VirtualKeyCode::Return) = ctx.key {
                    self.mode = InterfaceMode::Game;
                } else {
                    self.gui.clear_screen();
                    self.gui.draw_box(5, 5, 30, 8);
                    self.gui.center(6, &"WELCOME TO RASCAL!");
                    self.gui.center(8, &"EXPLORE THE DUNGEON");
                    self.gui.center(9, &"PULL THE FORBIDDEN LEVER");
                    self.gui.center(10, &"AND ESCAPE ALIVE");
                    self.gui.center(12, &"PRESS {18}RETURN{146} TO BEGIN");
                    for col in 0..6 {
                        self.gui.set(col + 15, 11, 100);
                    }
                }
            }
            InterfaceMode::Game => {
                self.game.run(&mut self.gui, &mut self.ecs);
                let offset = self.game.map_offset();
                {
                    let renderables = self.ecs.read_storage::<Renderable>();
                    let positions = self.ecs.read_storage::<Position>();

                    for (r, p) in (&renderables, &positions).join() {
                        if self.game.can_see(p.loc.x, p.loc.y) {
                            self.gui.set(
                                (p.loc.x + offset.0) as usize,
                                (p.loc.y + offset.1) as usize,
                                r.glyph,
                            );
                        }
                    }
                }

                if let Some(key) = ctx.key {
                    if let Some(mode) = self.game.handle_key(&mut self.ecs, key) {
                        self.mode = mode;
                    } else {
                        self.run_systems();
                    }
                }
            }
        }

        self.gui.draw(ctx);
    }
}

impl Interface {
    pub fn new() -> Interface {
        let mut ecs = World::new();

        register_components(&mut ecs);

        let game = Game::new();
        let loc = game.initial_loc();

        let player = ecs
            .create_entity()
            .with(Player {})
            .with(Renderable { glyph: 0 })
            .with(Position { loc })
            .build();
        ecs.insert(player);

        Interface {
            gui: Gui::new(),
            game,
            ecs,
            mode: InterfaceMode::Greeting,
        }
    }

    fn run_systems(&mut self) {
        let mut ips = ItemPickupSystem {};
        ips.run_now(&self.ecs);

        self.ecs.maintain();
    }
}
