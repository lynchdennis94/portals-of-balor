use bracket_lib::terminal::{BTerm, GameState};

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}
fn main() -> bracket_lib::terminal::BError {
    use bracket_lib::terminal::BTermBuilder;
    let context: BTerm = BTermBuilder::simple80x50()
        .with_title("Portals of Balor")
        .build()?;
    let gs: State = State {};
    bracket_lib::terminal::main_loop(context, gs)
}
