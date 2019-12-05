mod parser;

use futures::executor::block_on;
use crossbeam_channel::unbounded;
use notify::{ RecursiveMode, Result, watcher, Watcher, EventKind };
use notify::{ Error, ErrorKind };
use std::time::Duration;



fn main() -> Result<()> {

    let directories = vec!["a", "b", "c"];
    let files = block_on(parser::run(directories));





    // Create a channel to receive the events
    let (tx, rx) = unbounded();

    // Automatically select the best implementation for this platform
    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

    // Add path to be watched, recursively so everything inside it's children
    // and their children gets watched.
    watcher.watch("/Users/alex.hexan/repo/journal_sys/sys", RecursiveMode::Recursive);

    loop {
        // Find out if the incoming event went ok
        let event = match rx.recv() {
            Ok(e) => e,
            Err(err) => Err(Error::generic("Watcher error"))
        };


        // Find out what kind the event is, if it's an error Kind::Any works
        let event_kind = match event {
            Ok(e) => e.kind,
            Err(err) => EventKind::Any
        };


        // Match the event kind and run the appropriate functions for it
        match event_kind {
            EventKind::Modify(kind) => println!("\t\tFile changed\n\n"),
            EventKind::Create(kind) => println!("File created\n\n"),
            EventKind::Remove(kind) => println!("File removed\n\n"),
            _ => println!("Something else happened, probably an error")
        }
    }
}
