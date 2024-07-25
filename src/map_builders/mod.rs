use crate::Position;

use super::Map;
mod simple_map;
use simple_map::*;
use specs::World;
mod common;

pub trait MapBuilder {
    fn build_map(&mut self) -> (Map, Position);
    fn spawn_entities(&mut self, map: &Map, ecs: &mut World);
}

pub fn random_builder() -> Box<dyn MapBuilder> {
    Box::new(SimpleMapBuilder::new())
}
