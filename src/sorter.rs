use std::env::args;
use std::fs;
use std::io::Error;
use std::path::Path;

pub fn sort(path: &Path, target_dir: &Path) {
    let path = Path::new(&path);
    for entry in path.read_dir().expect("Read directory failed") {
        match entry {
            Ok(entry) => sort_path(&entry.path(), target_dir),
            Err(e) => eprintln!("Error occured during read of : {:?}", e),
        }
    }
}

pub fn sort_path(path: &Path, target_dir: &Path) {
    println!("Sort_path {:?}", path);

    let file_name = path
        .file_name()
        .expect("Extract file name")
        .to_str()
        .expect("Own string");
    if !file_name.starts_with("[SubsPlease]") {
        println!("Will not sort path: {:?}", path);
        return;
    }

    if path.is_file() {
        sort_file(path, target_dir);
    } else if path.is_dir() {
        sort_dir(path, target_dir);
    }
}

fn sort_dir(path: &Path, target_dir: &Path) {
    for entry in path.read_dir().expect("Read directory failed") {
        match entry {
            Ok(entry) => sort_path(&entry.path(), target_dir),
            Err(e) => eprintln!("Error occured during read of : {:?}", e),
        }
    }
    match fs::remove_dir(path) {
        Ok(_) => println!("Deleted empty dir {:?}", path),
        Err(e) => println!("Did not delete dir: {:?}, because: {:?}", path, e),
    }
}

fn sort_file(source: &Path, target_dir: &Path) {
    let file_name = source
        .file_stem()
        .expect("File needs valid stem")
        .to_str()
        .unwrap()
        .replace("[SubsPlease] ", "");
    let name_cleaned = file_name.split(" -").next().unwrap();
    let (season, name) = resolve_season_name(name_cleaned);
    let destination = target_dir
        .join(name)
        .join(season)
        .join(source.file_name().unwrap());
    rename_or_copydel(source, &destination)
}

fn rename_or_copydel(source: &Path, dest: &Path) {
    fs::create_dir_all(dest.parent().unwrap()).expect("create dir all failed: ");
    match fs::rename(source, dest) {
        Err(e) => copy_delete(e, source, dest),
        _ => println!("Moved file: {:?} to {:?}", source, dest),
    }
}

fn copy_delete(error: Error, source: &Path, dest: &Path) {
    eprintln!("Error moving file: {:?}", error);
    match fs::copy(source, dest) {
        Ok(_) => fs::remove_file(source).expect("can remove file after copy"),
        Err(e) => eprint!("Could not copy because: {:?}", e),
    }
}

fn resolve_season_name(name: &str) -> (&str, String) {
    // Is this good? No. But fuck it, this is quick poc
    if name.contains("S2") {
        ("Season 02", name.replace(" S2", ""))
    } else if name.contains("S3") {
        return ("Season 03", name.replace(" S3", ""));
    } else if name.contains("S4") {
        return ("Season 04", name.replace(" S4", ""));
    } else {
        return ("Season 01", name.replace(" S1", ""));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resolve_season_name_without_s1() {
        let (season, name) = resolve_season_name("stuff");
        assert_eq!(name, "stuff");
        assert_eq!(season, "Season 01")
    }

    #[test]
    fn test_resolve_season_name_with_s1() {
        let (season, name) = resolve_season_name("stuff S1");
        assert_eq!(name, "stuff");
        assert_eq!(season, "Season 01")
    }

    #[test]
    fn test_resolve_season_name_with_s2() {
        let (season, name) = resolve_season_name("stuff S2");
        assert_eq!(name, "stuff");
        assert_eq!(season, "Season 02")
    }
}
