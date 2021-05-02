#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;

use eyre::{Report, Result};
use tetra::graphics::scaling::{ScalingMode, ScreenScaler};
use tetra::graphics::{self, BlendAlphaMode, BlendMode, Color};
use tetra::input::{self, Key};
use tetra::{Context, ContextBuilder, Event, State};

use crate::assets::Assets;

const GAME_NAME: &str = "Tetra";
const SCREEN_WIDTH: i32 = 320;
const SCREEN_HEIGHT: i32 = 180;

fn main() {
    run().unwrap_or_else(report_crash);
}

fn run() -> Result<()> {
    ContextBuilder::new(GAME_NAME, SCREEN_WIDTH * 4, SCREEN_HEIGHT * 4)
        .resizable(true)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

#[cfg(debug_assertions)]
fn report_crash(err: Report) {
    println!("{:?}", err);
}

#[cfg(not(debug_assertions))]
fn report_crash(err: Report) {
    use std::fs::File;
    use std::io::Write;

    let mut crash_log = File::create("./crash_log.txt").unwrap();

    write!(
        crash_log,
        "Oh no! {} has crashed.\n\nHere's the error message, if you want to report an issue:\n\n{:?}",
        GAME_NAME, err
    )
    .unwrap();
}

struct GameState {
    assets: Assets,
    scaler: ScreenScaler,
}

impl GameState {
    fn new(ctx: &mut Context) -> Result<GameState> {
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
