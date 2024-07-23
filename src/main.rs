use bracket_lib::terminal::{BError, BTerm, GameState, RGB};
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::Rect;
mod visibility_system;
pub use visibility_system::*;

pub struct State {
    pub ecs: World,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        player_input(self, ctx);
        self.run_systems();

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        vis.run_now(&self.ecs);
        self.ecs.maintain();
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
    gs.ecs.register::<Viewshed>();

    // Add shared data for the world
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();
    gs.ecs.insert(map);

    // Create some sample entities
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_x,
            y: player_y,
        })
        .with(Renderable {
            glyph: 0x40,
            fg: RGB::named(bracket_lib::terminal::YELLOW),
            bg: RGB::named(bracket_lib::terminal::BLACK),
        })
        .with(Player {})
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
        })
        .build();

    bracket_lib::terminal::main_loop(context, gs)
}
