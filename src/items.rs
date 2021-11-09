use bracket_terminal::prelude::*;
use specs::prelude::*;

use crate::components::{InBackpack, Name, Position, WantsToPickUp};
use crate::gui::Gui;

pub struct ItemPickupSystem;

impl<'a> System<'a> for ItemPickupSystem {
    type SystemData = (
        WriteStorage<'a, WantsToPickUp>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, InBackpack>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut wants_pickup, mut positions, mut backpack) = data;

        for pickup in wants_pickup.join() {
            positions.remove(pickup.entity);
            backpack
                .insert(
                    pickup.entity,
                    InBackpack {
                        owner: pickup.collected_by,
                    },
                )
                .expect("Unable to insert backpack entry");
        }

        wants_pickup.clear();
    }
}

#[derive(PartialEq, Copy, Clone)]
pub enum MenuResult {
    NoResponse,
    Cancel,
}

pub fn show_inventory(ecs: &World, gui: &mut Gui, key: Option<VirtualKeyCode>) -> MenuResult {
    let player_entity = ecs.fetch::<Entity>();
    let names = ecs.read_storage::<Name>();
    let backpack = ecs.read_storage::<InBackpack>();
    let (top, height) = (4, 20);

    gui.draw_box(0, top - 1, 39, height);
    gui.write_at(2, top - 1, &"INVENTORY");
    gui.write_at(20, top + height - 1, &"PRESS   TO CLOSE");
    gui.set(26, top + height - 1, 31);

    for (j, (_pack, name)) in (&backpack, &names)
        .join()
        .filter(|item| item.0.owner == *player_entity)
        .enumerate()
    {
        let row = top + j;
        gui.set(1, row, 40);
        gui.set(2, row, (j + 1) as u8);
        gui.set(3, row, 41);

        gui.write_at(4, row, &name.name.to_string());
    }

    if let Some(VirtualKeyCode::Left) = key {
        MenuResult::Cancel
    } else {
        MenuResult::NoResponse
    }
}
