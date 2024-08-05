use crate::{CombatStats, RunState, Viewshed, WantsToMelee};

use super::{Map, Player, Position, State, MAP_HEIGHT, MAP_WIDTH};
use bracket_lib::terminal::{BTerm, Point, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let players = ecs.write_storage::<Player>();
    let mut viewsheds = ecs.write_storage::<Viewshed>();
    let combat_stats = ecs.read_storage::<CombatStats>();
    let entities = ecs.entities();
    let mut wants_to_melee = ecs.write_storage::<WantsToMelee>();
    let map = ecs.fetch::<Map>();

    for (entity, _player, pos, viewshed) in
        (&entities, &players, &mut positions, &mut viewsheds).join()
    {
        let possible_x = pos.x + delta_x;
        let possible_y = pos.y + delta_y;
        if possible_x < 1
            || possible_x > map.width - 1
            || possible_y < 1
            || possible_y > map.height - 1
        {
            return;
        }
        let destination_idx = map.xy_idx(possible_x, possible_y);

        for potential_target in map.tile_content[destination_idx].iter() {
            let player_entity = players.get(*potential_target);
            if let Some(_player_target) = player_entity {
                continue; // We don't want to attack ourselves
            }
            let target = combat_stats.get(*potential_target);
            if let Some(_target) = target {
                wants_to_melee
                    .insert(
                        entity,
                        WantsToMelee {
                            target: *potential_target,
                        },
                    )
                    .expect("Add target failed");
                return;
            }
        }
        if !map.blocked[destination_idx] {
            pos.x = min(MAP_WIDTH - 1, max(0, possible_x));
            pos.y = min(MAP_HEIGHT - 1, max(0, possible_y));

            viewshed.dirty = true;
            let mut ppos = ecs.write_resource::<Point>();
            ppos.x = pos.x;
            ppos.y = pos.y;
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) -> RunState {
    match ctx.key {
        None => return RunState::AwaitingInput,
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::H | VirtualKeyCode::Numpad4 => {
                try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::K | VirtualKeyCode::Numpad8 => {
                try_move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::L | VirtualKeyCode::Numpad6 => {
                try_move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::J | VirtualKeyCode::Numpad2 => {
                try_move_player(0, 1, &mut gs.ecs)
            }
            VirtualKeyCode::Y | VirtualKeyCode::Numpad7 => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::U | VirtualKeyCode::Numpad9 => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::B | VirtualKeyCode::Numpad1 => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::N | VirtualKeyCode::Numpad3 => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad5 => try_move_player(0, 0, &mut gs.ecs),
            _ => return RunState::AwaitingInput,
        },
    }
    RunState::PlayerTurn
}
