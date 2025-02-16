mod sorter;
use sorter::sort;
use std::{
    env::{self, args},
    path::Path,
};

fn main() {
    let sort_path_str = match args().nth(1) {
        Some(path) => path,
        None => match env::var_os("UNCLEANED_PATH") {
            Some(path) => path.into_string().expect("Invalid path"),
            None => ".".to_string(),
        },
    };
    let target_dir_str = match args().nth(2) {
        Some(path) => path,
        None => match env::var_os("ANIME_PATH") {
            Some(path) => path.into_string().expect("Invalid path"),
            None => panic!("No target directory provided"),
        },
    };

    let sort_path = Path::new(&sort_path_str);
    let target_dir = Path::new(&target_dir_str);
    sort(sort_path, target_dir);
}
