#[macro_use]
extern crate lazy_static;

mod parser;

use futures::executor::block_on;
use crossbeam_channel::unbounded;
use notify::{ RecursiveMode, Result, watcher, Watcher, EventKind };
use notify::{ Error };
use std::time::Duration;



fn main() -> Result<()> {

    let directories = vec![
        "/Users/alex.hexan/repo/journal_sys/sys",
        "/Users/alex.hexan/repo/journal_sys/tests/unit"
    ];
    let files = block_on(parser::run(directories));


/*

    // Create a channel to receive the events
    let (tx, rx) = unbounded();

    // Automatically select the best implementation for this platform
    let mut watcher = watcher(tx, Duration::from_millis(100))?;

    // Add path to be watched, recursively so everything inside it's children
    // and their children gets watched.
    watcher.watch("/Users/alex.hexan/repo/journal_sys/sys", RecursiveMode::Recursive)?;

    loop {
        // Find out if the incoming event went ok
        let event = match rx.recv() {
            Ok(e) => e,
            Err(_) => Err(Error::generic("Watcher error"))
        };


        // Find out what kind the event is, if it's an error Kind::Any works
        let event_kind = match event {
            Ok(e) => e.kind,
            Err(_) => EventKind::Any
        };


        // Match the event kind and run the appropriate functions for it
        match event_kind {
            EventKind::Modify(_) => println!("\t\tFile changed\n\n"),
            EventKind::Create(_) => println!("File created\n\n"),
            EventKind::Remove(_) => println!("File removed\n\n"),
            _ => println!("Something else happened, probably an error")
        }
    }*/
    Ok(())
}
