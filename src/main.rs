use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, Color};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, Event, State};

const SCREEN_WIDTH: i32 = 320;
const SCREEN_HEIGHT: i32 = 180;

fn main() -> tetra::Result {
    ContextBuilder::new("Tetra", SCREEN_WIDTH * 4, SCREEN_HEIGHT * 4)
        .resizable(true)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

struct Assets {
    // assets go here...
}

impl Assets {
    fn load(ctx: &mut Context) -> tetra::Result<Assets> {
        Ok(Assets {})
    }
}

struct GameState {
    assets: Assets,
    scaler: ScreenScaler,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        Ok(GameState {
            assets: Assets::load(ctx)?,
            scaler: ScreenScaler::with_window_size(
                ctx,
                SCREEN_WIDTH,
                SCREEN_HEIGHT,
                ScalingMode::ShowAllPixelPerfect,
            )?,
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if input::is_key_pressed(ctx, Key::F5) {
            self.assets = Assets::load(ctx)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        // draw stuff here...

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);

        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: Event) -> tetra::Result {
        if let Event::Resized { width, height, .. } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }
}
