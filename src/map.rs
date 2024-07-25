use crate::MAP_COUNT;

use super::{Rect, MAP_HEIGHT, MAP_WIDTH};
use bracket_lib::{
    pathfinding::{Algorithm2D, BaseMap},
    prelude::Point,
    terminal::{BTerm, RGB},
};
use specs::World;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub struct Map {
    pub tiles: Vec<TileType>,
    pub rooms: Vec<Rect>,
    pub width: i32,
    pub height: i32,
    pub revealed_tiles: Vec<bool>,
    pub visible_tiles: Vec<bool>,
}

impl Map {
    pub fn xy_idx(&self, x: i32, y: i32) -> usize {
        (y as usize * MAP_WIDTH as usize) + x as usize
    }

    /// Generates an empty map, consisting entirely of solid walls
    pub fn new() -> Map {
        Map {
            tiles: vec![TileType::Wall; MAP_COUNT],
            rooms: Vec::new(),
            width: MAP_WIDTH,
            height: MAP_HEIGHT,
            revealed_tiles: vec![false; MAP_COUNT],
            visible_tiles: vec![false; MAP_COUNT],
        }
    }
}

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx] == TileType::Wall
    }
}

pub fn draw_map(ecs: &World, ctx: &mut BTerm) {
    let map = ecs.fetch::<Map>();

    let mut y = 0;
    let mut x = 0;
    for (idx, tile) in map.tiles.iter().enumerate() {
        if map.revealed_tiles[idx] {
            let glyph;
            let mut fg;
            match tile {
                TileType::Floor => {
                    fg = RGB::from_f32(0.5, 0.5, 0.5);
                    glyph = 0x2E;
                }
                TileType::Wall => {
                    fg = RGB::from_f32(0.0, 1.0, 0.0);
                    glyph = 0x23
                }
            }
            if !map.visible_tiles[idx] {
                fg = fg.to_greyscale();
            }
            ctx.set(x, y, fg, RGB::from_f32(0., 0., 0.), glyph)
        }

        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}
