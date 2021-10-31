#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::panic;
use std::process;
use std::thread;

fn main() {
    panic::set_hook(Box::new(|e| {
        // This should work on desktop, at least.
        if let Some("main") = thread::current().name() {
            let msg = e.to_string();
            report_error(&msg);
        }
    }));

    if let Err(e) = tetra_template::run() {
        let msg = format!("{:?}", e);
        report_error(&msg);
        process::exit(1);
    }
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
            tetra_template::GAME_NAME, msg
        )
        .unwrap();
    }
}
