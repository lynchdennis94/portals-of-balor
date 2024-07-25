use bracket_lib::random::RandomNumberGenerator;

use super::common::*;
use super::MapBuilder;
use crate::{Map, Rect, MAP_HEIGHT, MAP_WIDTH};

pub struct SimpleMapBuilder {}

impl MapBuilder for SimpleMapBuilder {
    fn build() -> Map {
        let mut map = Map::new();
        SimpleMapBuilder::rooms_and_corridors(&mut map);
        map
    }
}

impl SimpleMapBuilder {
    fn rooms_and_corridors(map: &mut Map) {
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, MAP_WIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, MAP_HEIGHT - h - 1) - 1;
            let new_room = Rect::new(x, y, w, h);
            let mut ok = true;
            for other_room in map.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                apply_room_to_map(map, &new_room);

                if !map.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = map.rooms[map.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(map, prev_x, new_x, new_y);
                    }
                }

                map.rooms.push(new_room);
            }
        }
    }
}
