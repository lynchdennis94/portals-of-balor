use crate::Position;

use super::Map;
mod simple_map;
use simple_map::*;
use specs::World;
mod common;

pub trait MapBuilder {
    fn build_map(&mut self);
    fn spawn_entities(&mut self, ecs: &mut World);
    fn get_map(&self) -> Map;
    fn get_starting_position(&self) -> Position;
}

pub fn random_builder() -> Box<dyn MapBuilder> {
    Box::new(SimpleMapBuilder::new())
}
