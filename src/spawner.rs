use crate::{Monster, Name, Position, Renderable, Viewshed};

use super::Rect;
use bracket_lib::{color::RGB, random::RandomNumberGenerator, terminal::FontCharType};
use specs::{Builder, World, WorldExt};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;
pub const MAP_COUNT: i32 = MAP_WIDTH * MAP_HEIGHT;

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
        .build();
}

pub fn spawn_room(ecs: &mut World, room: &Rect) {
    // Currently a 'dumbed down' version, spawns a single monster in center of rooms
    let (x, y) = room.center();
    random_monster(ecs, x, y);
}
