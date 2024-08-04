use crate::{Map, Position, WantsToMelee};

use super::{Monster, Viewshed};
use bracket_lib::terminal::Point;
use specs::prelude::*;

pub struct MonsterAI {}

impl<'a> System<'a> for MonsterAI {
    #[allow(clippy::type_complexity)]
    type SystemData = (
        WriteExpect<'a, Map>,
        ReadExpect<'a, Point>,
        ReadExpect<'a, Entity>,
        Entities<'a>,
        WriteStorage<'a, Viewshed>,
        ReadStorage<'a, Monster>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, WantsToMelee>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            mut map,
            player_pos,
            player_entity,
            entities,
            mut viewshed,
            monster,
            mut position,
            mut wants_to_melee,
        ) = data;
        for (entity, viewshed, _monster, pos) in
            (&entities, &mut viewshed, &monster, &mut position).join()
        {
            {
                let distance = bracket_lib::terminal::DistanceAlg::Pythagoras
                    .distance2d(Point::new(pos.x, pos.y), *player_pos);
                if distance < 1.5 {
                    wants_to_melee
                        .insert(
                            entity,
                            WantsToMelee {
                                target: *player_entity,
                            },
                        )
                        .expect("Unable to insert attack");
                } else if viewshed.visible_tiles.contains(&*player_pos) {
                    let path = bracket_lib::pathfinding::a_star_search(
                        map.xy_idx(pos.x, pos.y) as i32,
                        map.xy_idx(player_pos.x, player_pos.y) as i32,
                        &mut *map,
                    );

                    if path.success && path.steps.len() > 1 {
                        // Clear the 'blocked' status from the old spot
                        let old_idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[old_idx] = false;

                        // Update the position and viewshed of the monster
                        pos.x = path.steps[1] as i32 % map.width;
                        pos.y = path.steps[1] as i32 / map.width;
                        viewshed.dirty = true;

                        // Update the 'blocked' map with the new position
                        let new_idx = map.xy_idx(pos.x, pos.y);
                        map.blocked[new_idx] = true;
                    }
                }
            }
        }
    }
}
