use bracket_terminal::prelude::*;
use specs::prelude::*;

use crate::components::*;

use crate::game::*;
use crate::gui::*;

pub struct Interface {
    gui: Gui,
    game: Game,
    ecs: World,
    mode: InterfaceMode,
}

enum InterfaceMode {
    Greeting,
    Game,
}

impl GameState for Interface {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        match self.mode {
            InterfaceMode::Greeting => {
                if let Some(VirtualKeyCode::Q) = ctx.key {
                    self.mode = InterfaceMode::Game;
                } else {
                    self.gui.clear_screen();
                    self.gui.draw_box(5, 5, 30, 6);
                }
            }
            InterfaceMode::Game => {
                if let Some(key) = ctx.key {
                    self.game.handle_key(&mut self.ecs, key);
                    self.run_systems();
                }
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
        self.ecs.maintain();
    }
}
