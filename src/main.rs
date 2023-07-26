use std::{path::Path, time::Duration};
use notify::RecursiveMode;
use notify_debouncer_mini::new_debouncer;
use baz_daemon::should_notify;

extern crate env_logger;


/// Example for debouncer mini
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debouncer_mini=trace")).init();
    // setup debouncer
    let (tx, rx) = std::sync::mpsc::channel();

    // No specific tickrate, max debounce time 1 seconds
    let mut debouncer = new_debouncer(Duration::from_secs(1), Some(Duration::from_millis(100)), tx).unwrap();

    debouncer
        .watcher()
        .watch(Path::new("/Users/nimrodkor/Projects/2gitters"), RecursiveMode::Recursive)
        .unwrap();

    // print all events, non returning
    for result in rx {
        match result {
            Ok(events) => {
                println!("Got {} events", events.len());
                events.iter().for_each(|event| {
                    if should_notify(event) {
                        log::info!("Event {event:?}");
                    }
                })
            },
            Err(error) => log::info!("Error {error:?}"),
        }
    }
}