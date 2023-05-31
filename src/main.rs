mod sorter;
use notify::{RecursiveMode, Watcher, Event, Error, PollWatcher, Config};
use std::path::{Path, PathBuf};
use std::sync::mpsc::channel;
use std::env::{args};
use std::time::Duration;


fn main() {
    println!("Hello, world!");

    let mut args = args().skip(1);
    let watch_dir = args.next();
    let target_dir = args.next();

    match (watch_dir, target_dir) {
        (Some(w), Some(_)) => watch(Path::new(&w)),
        _ => panic!("Watch dir and target dir was not specified")
    }
}

fn watch(path: &Path) {
    let (sender, receiver) = channel();

    let config= Config::default()
    .with_poll_interval(Duration::from_secs(2))
    .with_compare_contents(true);

    let mut watcher = PollWatcher::new(sender, config).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(path, RecursiveMode::Recursive).unwrap();

    loop {
        
        match receiver.recv() {
            
           Ok(event) => handle_receive(event),
           Err(e) => println!("watch error: {:?}", e),
        }

    }
}

fn handle_receive(result: Result<Event, Error>){
    match result {
        Ok(event) => handle_event(event),
        Err(error) => println!("watch error: {:?}", error),
    }
}

fn handle_event(event: Event){
    match event.kind {
        notify::EventKind::Create(_) => handle_file(event.paths),
        notify::EventKind::Modify(_) => handle_file(event.paths),
        e => println!("Unhandled event: {:?}", e),
    }
}

fn handle_file(paths: Vec<PathBuf>){
    match paths.first(){
        Some(path) => sorter::sort_path(path),
        p => println!("No path found: {:?}", p)
    }
}
