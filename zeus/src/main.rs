use tetra::graphics::{self, Color};
use tetra::{Context, ContextBuilder, State};

fn main() -> tetra::Result {
    ContextBuilder::new("Pong", 640, 480)
        .show_mouse(true)
        .quit_on_escape(true)
        .build()?
        .run(|_ctx| Ok(GameState {}))
}

struct GameState {}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        Ok(())
    }
}