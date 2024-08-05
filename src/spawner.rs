use std::collections::HashMap;

use crate::{
    BlocksTile, CombatStats, Map, Monster, Name, Position, Renderable, TileType, Viewshed,
};

use super::Rect;
use bracket_lib::{color::RGB, random::RandomNumberGenerator, terminal::FontCharType};
use specs::{Builder, World, WorldExt};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const MAX_MONSTERS: i32 = 3;
pub const MAP_COUNT: usize = (MAP_WIDTH * MAP_HEIGHT) as usize;

pub fn random_monster(ecs: &mut World, x: i32, y: i32) {
    let monster_name: String;
    let glyph: FontCharType;
    let roll: i32;
    {
        let mut rng = RandomNumberGenerator::new();
        roll = rng.roll_dice(1, 2);
        match roll {
            1 => {
                glyph = 0x67;
                monster_name = "Goblin".to_string()
            }
            _ => {
                glyph = 0x6F;
                monster_name = "Orc".to_string()
            }
        }
    }
    ecs.create_entity()
        .with(Position { x, y })
        .with(Renderable {
            glyph: glyph,
            fg: RGB::named(bracket_lib::terminal::RED),
            bg: RGB::named(bracket_lib::terminal::BLACK),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .with(Monster {})
        .with(Name {
            name: monster_name.to_string(),
        })
        .with(BlocksTile {})
        .with(CombatStats {
            max_hp: 16,
            hp: 16,
            defense: 1,
            power: 4,
        })
        .build();
}

pub fn spawn_room(ecs: &mut World, room: &Rect) {
    let mut possible_targets: Vec<usize> = Vec::new();
    {
        let map = ecs.fetch::<Map>();
        for y in room.y1 + 1..room.y2 {
            for x in room.x1 + 1..room.x2 {
                let idx = map.xy_idx(x, y);
                if map.tiles[idx] == TileType::Floor {
                    possible_targets.push(idx);
                }
            }
        }
    }

    spawn_region(ecs, &possible_targets);
}

fn spawn_entity(ecs: &mut World, spawn: &(&usize, &String)) {
    let x = (*spawn.0 % MAP_WIDTH as usize) as i32;
    let y = (*spawn.0 / MAP_WIDTH as usize) as i32;

    // Eventually, replace this with a spawn table for depth
    random_monster(ecs, x, y);
}

pub fn spawn_region(ecs: &mut World, area: &[usize]) {
    let mut spawn_points: HashMap<usize, String> = HashMap::new();
    let mut areas: Vec<usize> = Vec::from(area);

    {
        let mut rng = RandomNumberGenerator::new();
        let num_spawns = i32::min(areas.len() as i32, rng.roll_dice(1, MAX_MONSTERS));
        if num_spawns == 0 {
            return;
        }

        for _i in 0..num_spawns {
            let array_index = if areas.len() == 1 {
                0usize
            } else {
                (rng.roll_dice(1, areas.len() as i32) - 1) as usize
            };
            let map_idx = areas[array_index];
            spawn_points.insert(map_idx, "".to_string()); // Eventually this will use a spawn table
            areas.remove(array_index);
        }
    }

    for spawn in spawn_points.iter() {
        spawn_entity(ecs, &spawn);
    }
}
