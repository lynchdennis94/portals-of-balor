use super::Map;
mod simple_map;
use simple_map::*;
mod common;

trait MapBuilder {
    fn build() -> Map;
}

pub fn build_random_map() -> Map {
    SimpleMapBuilder::build()
}
