use rltk::{console, Point};
use specs::prelude::*;

use crate::components::{Monster, Name, Viewshed};

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    type SystemData = (
        ReadStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadExpect<'a, Point>,
        ReadStorage<'a, Name>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (viewshed, monster, player_pos, name) = data;

        for (viewshed, _monster, name) in (&viewshed, &monster, &name).join() {
            if viewshed.visible_tiles.contains(&*player_pos) {
                console::log(&format!("{} shouts insults", name.name));
            }
        }
    }
}
