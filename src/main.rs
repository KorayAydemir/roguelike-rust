use bracket_lib::terminal::{BTerm, BTermBuilder, GameState};
struct State {}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print(1, 1, "hi");
    }
}

fn main() {
    let context = BTermBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()
        .unwrap();

    let gs = State {};

    bracket_lib::terminal::main_loop(context, gs);
}
