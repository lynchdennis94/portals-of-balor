use crate::Position;

use super::Map;
mod simple_map;
use bracket_lib::random::RandomNumberGenerator;
use simple_map::*;
mod bsp_dungeon;
use bsp_dungeon::*;
use specs::World;
mod common;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
}

pub fn random_builder() -> Box<dyn MapBuilder> {
    let mut rng = RandomNumberGenerator::new();
    let builder_idx = rng.roll_dice(1, 2);
    match builder_idx {
        1 => Box::new(SimpleMapBuilder::new()),
        _ => Box::new(BspDungeonBuilder::new()),
    }
}
