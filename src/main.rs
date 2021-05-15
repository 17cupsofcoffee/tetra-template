#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod game;

use std::panic;

use eyre::Result;
use tetra::ContextBuilder;

use crate::game::GameState;

const GAME_NAME: &str = "Tetra";
const SCREEN_WIDTH: i32 = 320;
const SCREEN_HEIGHT: i32 = 180;

fn main() {
    panic::set_hook(Box::new(|e| {
        let msg = e.to_string();
        report_error(&msg);
    }));

    if let Err(e) = run() {
        let msg = format!("{:?}", e);
        report_error(&msg);
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    ContextBuilder::new(GAME_NAME, SCREEN_WIDTH * 4, SCREEN_HEIGHT * 4)
        .resizable(true)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}

fn report_error(msg: &str) {
    #[cfg(debug_assertions)]
    {
        println!("{}", msg);
    }

    #[cfg(not(debug_assertions))]
    {
        use std::fs::File;
        use std::io::Write;

        let mut crash_log = File::create("./crash_log.txt").unwrap();

        write!(
            crash_log,
            "Oh no! {} has crashed.\n\nHere's the error message, if you want to report the issue:\n\n{}",
            GAME_NAME, msg
        )
        .unwrap();
    }
}
