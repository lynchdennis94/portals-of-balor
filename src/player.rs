use super::{xy_idx, Player, Position, State, TileType, MAP_HEIGHT, MAP_WIDTH};
use bracket_lib::terminal::{BTerm, VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{max, min};

pub fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let possible_x = pos.x + delta_x;
        let possible_y = pos.y + delta_y;
        let destination_idx = xy_idx(possible_x, possible_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(MAP_WIDTH - 1, max(0, possible_x));
            pos.y = min(MAP_HEIGHT - 1, max(0, possible_y));
        }
    }
}

pub fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
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
            _ => {}
        },
    }
}
