use bracket_lib::random::RandomNumberGenerator;

use super::{common::*, MapBuilder};
use crate::{spawner, Map, Position, Rect, MAP_HEIGHT, MAP_WIDTH};

pub struct SimpleMapBuilder {
    map: Map,
    starting_position: Position,
    rooms: Vec<Rect>,
}

impl MapBuilder for SimpleMapBuilder {
    fn build_map(&mut self) {
        SimpleMapBuilder::rooms_and_corridors(self);
    }

    fn spawn_entities(&mut self, ecs: &mut specs::World) {
        ecs.insert(self.get_map());
        for room in self.rooms.iter().skip(1) {
            spawner::spawn_room(ecs, room);
        }
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }
}

impl SimpleMapBuilder {
    pub fn new() -> SimpleMapBuilder {
        println!("Using Simple Map Builder");
        SimpleMapBuilder {
            map: Map::new(),
            starting_position: Position { x: 0, y: 0 },
            rooms: Vec::new(),
        }
    }

    fn rooms_and_corridors(&mut self) {
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
            for other_room in self.rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                apply_room_to_map(&mut self.map, &new_room);

                if !self.rooms.is_empty() {
                    let (new_x, new_y) = new_room.center();
                    let (prev_x, prev_y) = self.rooms[self.rooms.len() - 1].center();
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut self.map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut self.map, prev_x, new_x, new_y);
                    }
                }

                self.rooms.push(new_room);
            }
        }

        let start_pos = self.rooms[0].center();
        self.starting_position = Position {
            x: start_pos.0,
            y: start_pos.1,
        };
    }
}
