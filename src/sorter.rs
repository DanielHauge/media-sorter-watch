use std::fs;
use std::{path::PathBuf, env::args};
use std::path::{Path};

pub fn sort_path(path: &PathBuf){
    println!("Sort_path");

    let file_name=path.file_name().unwrap().to_str().unwrap();
    if !file_name.starts_with("[SubsPlease]") { 
        println!("Will not sort path: {:?}", path);
        return; 
    }

    if path.is_file() {
        sort_file(path)
    } else if path.is_dir(){
        sort_dir(path)
    }
}

fn sort_dir(path: &PathBuf){
    for entry in path.read_dir().expect("Read directory failed"){
        match entry {
            Err(e) => println!("Error occured during read of : {:?}", e),
            Ok(entry) => sort_path(&entry.path())
        }
    }
}

fn sort_file(source: &PathBuf){

    let file_name = source.file_stem().unwrap().to_str().unwrap().replace("[SubsPlease] ", "");
    let name_cleaned = file_name.split(" -").next().unwrap();
    let (season,name) = resolve_season(name_cleaned);
    let target_dir = args().skip(2).next().expect("there should be atleast 3 runtime args");
    let destination= Path::new(&target_dir).join(name).join(season).join(source.file_name().unwrap());
    fs::create_dir_all(destination.parent().unwrap()).expect("create dir all failed: ");
    // Using copy->remove as move/rename assumes atomic operation which is not possible for cross-device linking.
    fs::copy(source, destination).expect("could not copy");
    fs::remove_file(source).expect("could not remove old file");
}

fn resolve_season(name: &str) -> (&str, String){
    // Is this good? No. But fuck it, this is quick poc
    if name.contains("S2"){
        return ("Season 02", name.replace(" S2", ""))
    } else if name.contains("S3"){
        return ("Season 03", name.replace(" S3", ""))
    } else if name.contains("S4"){
        return ("Season 04", name.replace(" S4", ""))
    } else {
        return ("Season 01", name.replace(" S1", ""));
    }
}


