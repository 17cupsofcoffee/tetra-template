use eyre::{Report, Result};
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, BlendAlphaMode, BlendMode, Color};
use tetra::input::{self, Key};
use tetra::{Context, Event, State};

use crate::assets::Assets;
use crate::{SCREEN_HEIGHT, SCREEN_WIDTH};

pub struct GameState {
    assets: Assets,
    scaler: ScreenScaler,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> Result<GameState> {
        graphics::set_blend_mode(ctx, BlendMode::Alpha(BlendAlphaMode::Premultiplied));

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

impl State<Report> for GameState {
    fn update(&mut self, ctx: &mut Context) -> Result<()> {
        if input::is_key_pressed(ctx, Key::F5) {
            self.assets = Assets::load(ctx)?;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<()> {
        graphics::set_canvas(ctx, self.scaler.canvas());
        graphics::clear(ctx, Color::rgb(0.392, 0.584, 0.929));

        // draw stuff here...

        graphics::reset_canvas(ctx);
        graphics::clear(ctx, Color::BLACK);
        self.scaler.draw(ctx);

        Ok(())
    }

    fn event(&mut self, _: &mut Context, event: Event) -> Result<()> {
        if let Event::Resized { width, height, .. } = event {
            self.scaler.set_outer_size(width, height);
        }

        Ok(())
    }
}
