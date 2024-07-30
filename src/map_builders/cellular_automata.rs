use std::collections::HashMap;

use bracket_lib::random::RandomNumberGenerator;

use super::MapBuilder;
use crate::{spawner, Map, Position, TileType};
pub struct CellularAutomataBuilder {
    map: Map,
    starting_position: Position,
    noise_areas: HashMap<i32, Vec<usize>>,
}

impl MapBuilder for CellularAutomataBuilder {
    fn build_map(&mut self) {
        self.build()
    }

    fn spawn_entities(&mut self, ecs: &mut specs::World) {
        for area in self.noise_areas.iter() {
            spawner::spawn_region(ecs, area.1);
        }
    }

    fn get_map(&self) -> Map {
        self.map.clone()
    }

    fn get_starting_position(&self) -> Position {
        self.starting_position.clone()
    }
}

impl CellularAutomataBuilder {
    pub fn new() -> CellularAutomataBuilder {
        println!("Using the Cellular Automata Builder");
        CellularAutomataBuilder {
            map: Map::new(),
            starting_position: Position { x: 0, y: 0 },
            noise_areas: HashMap::new(),
        }
    }

    fn build(&mut self) {
        let mut rng = RandomNumberGenerator::new();

        // First, completely randomize the map, setting 55% to floor
        for y in 1..self.map.height - 1 {
            for x in 1..self.map.width - 1 {
                let roll = rng.roll_dice(1, 100);
                let idx = self.map.xy_idx(x, y);
                if roll > 55 {
                    self.map.tiles[idx] = TileType::Wall
                } else {
                    self.map.tiles[idx] = TileType::Floor
                }
            }
        }

        // Now, iteratively apply cellular automata rules
        for _i in 0..15 {
            let mut newtiles = self.map.tiles.clone();

            for y in 1..self.map.height - 1 {
                for x in 1..self.map.width - 1 {
                    let idx = self.map.xy_idx(x, y);
                    let mut neighbors = 0;

                    // calculate neighbors - uses an array of neighbor indices, instead
                    // of the if statement blocks from the tutorial
                    let neighbor_index_array = [
                        idx - 1,
                        idx + 1,
                        idx - self.map.width as usize,
                        idx + self.map.width as usize,
                        idx - (self.map.width as usize - 1),
                        idx - (self.map.width as usize + 1),
                        idx + (self.map.width as usize - 1),
                        idx + (self.map.width as usize + 1),
                    ];
                    for neighbor_index in neighbor_index_array {
                        if self.map.tiles[neighbor_index] == TileType::Wall {
                            neighbors += 1;
                        }
                    }

                    if neighbors > 4 || neighbors == 0 {
                        newtiles[idx] = TileType::Wall;
                    } else {
                        newtiles[idx] = TileType::Floor;
                    }
                }
            }

            self.map.tiles = newtiles.clone();

            self.starting_position = Position {
                x: self.map.width / 2,
                y: self.map.height / 2,
            };
            let mut start_idx = self
                .map
                .xy_idx(self.starting_position.x, self.starting_position.y);
            while self.map.tiles[start_idx] != TileType::Floor {
                self.starting_position.x -= 1;
                start_idx = self
                    .map
                    .xy_idx(self.starting_position.x, self.starting_position.y);
            }
        }
        // TODO: Calculate exit tile spot

        // Build a noise map for spawning entities
        let mut noise = bracket_lib::noise::FastNoise::seeded(rng.roll_dice(1, 65536) as u64);
        noise.set_noise_type(bracket_lib::noise::NoiseType::Cellular);
        noise.set_frequency(0.08);
        noise.set_cellular_distance_function(
            bracket_lib::noise::CellularDistanceFunction::Manhattan,
        );

        for y in 1..self.map.height - 1 {
            for x in 1..self.map.width - 1 {
                let idx = self.map.xy_idx(x, y);
                if self.map.tiles[idx] == TileType::Floor {
                    let cell_value_f = noise.get_noise(x as f32, y as f32) * 10240.0;
                    let cell_value = cell_value_f as i32;

                    if self.noise_areas.contains_key(&cell_value) {
                        self.noise_areas.get_mut(&cell_value).unwrap().push(idx);
                    } else {
                        self.noise_areas.insert(cell_value, vec![idx]);
                    }
                }
            }
        }
    }
}
