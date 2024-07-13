use bracket_lib::terminal::{BError, BTerm, GameState, VirtualKeyCode, RGB};
use specs::prelude::*;
use specs_derive::Component;
use std::cmp::{max, min};

const MAP_WIDTH: i32 = 80;
const MAP_HEIGHT: i32 = 50;

struct State {
    ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        self.ecs.maintain();
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: bracket_lib::terminal::FontCharType,
    fg: RGB,
    bg: RGB,
}

#[derive(Component, Debug)]
struct Player {}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>();

    for (_player, pos) in (&mut players, &mut positions).join() {
        let possible_x = pos.x + delta_x;
        let possible_y = pos.y + delta_y;
        let destination_idx = xy_idx(possible_x, possible_y);
        if map[destination_idx] != TileType::Wall {
            pos.x = min(MAP_WIDTH - 1, max(0, possible_x));
            pos.y = min(MAP_HEIGHT - 1, max(0, possible_y));
        }
    }
}

fn player_input(gs: &mut State, ctx: &mut BTerm) {
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left | VirtualKeyCode::H | VirtualKeyCode::Numpad4 => {
                try_move_player(-1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Up | VirtualKeyCode::K | VirtualKeyCode::Numpad8 => {
                try_move_player(0, -1, &mut gs.ecs)
            }
            VirtualKeyCode::Right | VirtualKeyCode::L | VirtualKeyCode::Numpad6 => {
                try_move_player(1, 0, &mut gs.ecs)
            }
            VirtualKeyCode::Down | VirtualKeyCode::J | VirtualKeyCode::Numpad2 => {
                try_move_player(0, 1, &mut gs.ecs)
            }
            VirtualKeyCode::Y | VirtualKeyCode::Numpad7 => try_move_player(-1, -1, &mut gs.ecs),
            VirtualKeyCode::U | VirtualKeyCode::Numpad9 => try_move_player(1, -1, &mut gs.ecs),
            VirtualKeyCode::B | VirtualKeyCode::Numpad1 => try_move_player(-1, 1, &mut gs.ecs),
            VirtualKeyCode::N | VirtualKeyCode::Numpad3 => try_move_player(1, 1, &mut gs.ecs),
            VirtualKeyCode::Numpad5 => try_move_player(0, 0, &mut gs.ecs),
            _ => {}
        },
    }
}

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor,
}

pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAP_WIDTH as usize) + x as usize
}

fn new_map() -> Vec<TileType> {
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

fn draw_map(map: &[TileType], ctx: &mut BTerm) {
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

fn main() -> BError {
    use bracket_lib::terminal::BTermBuilder;
    let context: BTerm = BTermBuilder::simple(MAP_WIDTH, MAP_HEIGHT)
        .expect("Could not construct BTermBuilder")
        .with_title("Portals of Balor")
        .with_tile_dimensions(16, 16)
        .build()?;
    let mut gs: State = State { ecs: World::new() };

    // Register components to the world
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    // Add shared data for the world
    gs.ecs.insert(new_map());

    // Create some sample entities
    gs.ecs
        .create_entity()
        .with(Position { x: 40, y: 25 })
        .with(Renderable {
            glyph: 0x40,
            fg: RGB::named(bracket_lib::terminal::YELLOW),
            bg: RGB::named(bracket_lib::terminal::BLACK),
        })
        .with(Player {})
        .build();

    bracket_lib::terminal::main_loop(context, gs)
}
