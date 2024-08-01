use crate::{Map, Position};

use super::{Monster, Name, Viewshed};
use bracket_lib::terminal::{console, Point};
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        ReadStorage<'a, Name>,
        WriteStorage<'a, Position>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (mut map, player_pos, mut viewshed, monster, name, mut position) = data;

        for (viewshed, _monster, name, pos) in
            (&mut viewshed, &monster, &name, &mut position).join()
        {
            let distance = bracket_lib::terminal::DistanceAlg::Pythagoras
                .distance2d(Point::new(pos.x, pos.y), *player_pos);
            if distance < 1.5 {
                // Attack goes here
                console::log(&format!("{} shouts insults", name.name));
                return;
            }
            if viewshed.visible_tiles.contains(&*player_pos) {
                let path = bracket_lib::pathfinding::a_star_search(
                    map.xy_idx(pos.x, pos.y) as i32,
                    map.xy_idx(player_pos.x, player_pos.y) as i32,
                    &mut *map,
                );

                // Bug? Monsters still stack up on each other
                if path.success && path.steps.len() > 1 {
                    pos.x = path.steps[1] as i32 % map.width;
                    pos.y = path.steps[1] as i32 / map.width;
                    viewshed.dirty = true;
                }
            }
        }
    }
}
