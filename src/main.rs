use ggez::graphics::Color;
use ggez::{Context, ContextBuilder, GameResult, graphics};
use ggez::event::{self, EventHandler};

struct Game;

fn main() {
    let (ctx, event_loop) =
        ContextBuilder::new("Game", "Kallekål")
        .build()
        .expect("Could not initialize game :(");

    let game = Game;
    event::run(ctx, event_loop, game);
}

impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let canvas = graphics::Canvas::from_frame(ctx, Color::RED);
        canvas.finish(ctx)
    }
}

