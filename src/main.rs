mod parser;

use futures::executor::block_on;
use crossbeam_channel::unbounded;
use notify::{ RecursiveMode, Result, watcher, Watcher, EventKind };
use notify::{ Error, ErrorKind };
use std::time::Duration;



fn main() -> Result<()> {

    let directories = vec!["a", "b", "c"];
    let files = block_on(parser::run(directories));




    let (tx, rx) = unbounded();

    let mut watcher = watcher(tx, Duration::from_millis(100)).unwrap();

    watcher.watch("/Users/alex.hexan/repo/journal_sys/sys", RecursiveMode::Recursive);


    loop {
        let event = match rx.recv() {
            Ok(e) => e,
            Err(err) => Err(Error::generic("Shit broke"))
        };


        let event_kind = match event {
            Ok(e) => e.kind,
            Err(err) => EventKind::Any
        };


        match event_kind {
            EventKind::Modify(kind) => println!("\t\tFile changed\n\n"),
            EventKind::Create(kind) => println!("File created\n\n"),
            EventKind::Remove(kind) => println!("File removed\n\n"),
            _ => println!("Something else happened, probably an error")
        }
    }
}
