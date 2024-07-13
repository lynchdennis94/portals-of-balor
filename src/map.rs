use bracket_lib::terminal::{BTerm, RGB};

pub const MAP_WIDTH: i32 = 80;
pub const MAP_HEIGHT: i32 = 50;

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAP_WIDTH as usize) + x as usize
}

pub fn new_map() -> Vec<TileType> {
    let mut map = vec![TileType::Floor; (MAP_WIDTH * MAP_HEIGHT) as usize];

    for x in 0..MAP_WIDTH {
        map[xy_idx(x, 0)] = TileType::Wall;
        map[xy_idx(x, MAP_HEIGHT - 1)] = TileType::Wall;
    }

    for y in 0..MAP_HEIGHT {
        map[xy_idx(0, y)] = TileType::Wall;
        map[xy_idx(MAP_WIDTH - 1, y)] = TileType::Wall;
    }

    // Get RNG to splatter some walls around!
    let mut rng = bracket_lib::random::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, MAP_WIDTH - 1);
        let y = rng.roll_dice(1, MAP_HEIGHT - 1);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = TileType::Wall;
        }
    }

    map
}

pub fn draw_map(map: &[TileType], ctx: &mut BTerm) {
    let mut y = 0;
    let mut x = 0;

    for tile in map.iter() {
        match tile {
            TileType::Floor => ctx.set(
                x,
                y,
                RGB::from_f32(0.5, 0.5, 0.5),
                RGB::from_f32(0., 0., 0.),
                0x2E,
            ),
            TileType::Wall => ctx.set(
                x,
                y,
                RGB::from_f32(0.0, 1.0, 0.0),
                RGB::from_f32(0., 0., 0.),
                0x23,
            ),
        }

        x += 1;
        if x >= MAP_WIDTH {
            x = 0;
            y += 1;
        }
    }
}
