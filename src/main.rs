use bracket_lib::{
    random::RandomNumberGenerator,
    terminal::{BError, BTerm, FontCharType, GameState, Point, RGB},
};
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
mod monster_ai_system;
pub use monster_ai_system::*;

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}

pub struct State {
    pub ecs: World,
    pub runstate: RunState,
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();

        if self.runstate == RunState::Running {
            self.run_systems();
            self.runstate = RunState::Paused;
        } else {
            self.runstate = player_input(self, ctx);
        }

        draw_map(&self.ecs, ctx);

        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();
        let map = self.ecs.fetch::<Map>();

        for (pos, render) in (&positions, &renderables).join() {
            let idx = map.xy_idx(pos.x, pos.y);
            if map.visible_tiles[idx] {
                ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
            }
        }
    }
}

impl State {
    fn run_systems(&mut self) {
        let mut vis = VisibilitySystem {};
        let mut mob = MonsterAI {};
        vis.run_now(&self.ecs);
        mob.run_now(&self.ecs);
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
    let mut gs: State = State {
        ecs: World::new(),
        runstate: RunState::Running,
    };

    // Register components to the world
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();
    gs.ecs.register::<Viewshed>();
    gs.ecs.register::<Monster>();
    gs.ecs.register::<Name>();

    // Add shared data for the world
    let map = Map::new_map_rooms_and_corridors();
    let (player_x, player_y) = map.rooms[0].center();

    // Create the player entity
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
        .with(Name {
            name: "Player".to_string(),
        })
        .with(Viewshed {
            visible_tiles: Vec::new(),
            range: 8,
            dirty: true,
        })
        .build();

    // Create some monster entities
    let mut rng = RandomNumberGenerator::new();
    for room in map.rooms.iter().skip(1) {
        let (x, y) = room.center();
        let glyph: FontCharType;
        let monster_name: String;
        let roll = rng.roll_dice(1, 2);
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
        gs.ecs
            .create_entity()
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

    gs.ecs.insert(Point::new(player_x, player_y));
    gs.ecs.insert(map);

    bracket_lib::terminal::main_loop(context, gs)
}
