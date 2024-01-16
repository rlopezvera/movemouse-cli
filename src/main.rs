use clap::Parser;
use enigo::*;
use std::{thread, time};

#[derive(Parser)]
struct Cli {
    #[clap(short, long, default_value = "3")]
    interval: u64,

    #[clap(short, long, default_value = "10000")]
    duration: u64,
}

fn move_cursor(interval: u64, mut duration: u64) {
    let mut enigo = Enigo::new();

    // loop for the duration of duration in minutes

    duration = duration * 60;

    let mut i = 0;

    while i < duration {
        enigo.mouse_move_relative(100, 0);
        thread::sleep(time::Duration::from_secs(1));
        enigo.mouse_move_relative(0, 100);
        thread::sleep(time::Duration::from_secs(1));
        enigo.mouse_move_relative(-100, 0);
        thread::sleep(time::Duration::from_secs(1));
        enigo.mouse_move_relative(0, -100);
        thread::sleep(time::Duration::from_secs(1));
        thread::sleep(time::Duration::from_secs(interval));
        i += interval+4;
    }
}

fn main() {
    let args = Cli::parse();

    move_cursor(args.interval, args.duration);
}
