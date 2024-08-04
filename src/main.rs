use bracket_lib::terminal::{BError, BTerm, GameState, Point, RGB};
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
mod map_indexing_system;
pub use map_indexing_system::*;
mod melee_combat_system;
pub use melee_combat_system::*;
mod damage_system;
pub use damage_system::*;
mod spawner;
pub use spawner::*;
pub mod map_builders;

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
            damage_system::delete_the_dead(&mut self.ecs);
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
        let mut mapindex = MapIndexingSystem {};
        let mut melee = MeleeCombatSystem {};
        let mut damagesystem = DamageSystem {};
        vis.run_now(&self.ecs);
        mob.run_now(&self.ecs);
        mapindex.run_now(&self.ecs);
        melee.run_now(&self.ecs);
        damagesystem.run_now(&self.ecs);
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
    gs.ecs.register::<BlocksTile>();
    gs.ecs.register::<CombatStats>();
    gs.ecs.register::<SufferDamage>();
    gs.ecs.register::<WantsToMelee>();

    // Add shared data for the world
    let mut builder = map_builders::random_builder();
    builder.build_map();
    let map = builder.get_map();
    let player_start = builder.get_starting_position();

    // Create the player entity
    gs.ecs
        .create_entity()
        .with(Position {
            x: player_start.x,
            y: player_start.y,
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
        .with(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .build();

    // Create some monster entities
    builder.spawn_entities(&mut gs.ecs);

    gs.ecs.insert(Point::new(player_start.x, player_start.y));
    gs.ecs.insert(map);

    bracket_lib::terminal::main_loop(context, gs)
}
