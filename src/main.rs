mod transform;

use colored::Colorize;
use notify::{watcher, RecursiveMode, Watcher};
use std::sync::mpsc::channel;
use std::time::Duration;
use transform::update;

fn main() {
    println!(
        "{}",
        "[watch] Elde started, watching for changes in /org ...".bright_magenta()
    );

    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    watcher.watch("./org", RecursiveMode::Recursive).unwrap();

    loop {
        match rx.recv() {
            Ok(event) => {
                match event {
                    notify::DebouncedEvent::NoticeWrite(path) => transform::update(path),
                    notify::DebouncedEvent::Create(path) => transform::update(path),
                    _ => {}
                };
            }
            Err(e) => println!("Error trying to watch event ..."),
        };
    }
}
