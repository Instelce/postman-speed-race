use std::{
    env::current_dir,
    fs::{self, read_to_string},
    io,
    path::Path,
};

pub fn get_root_dir(dirname: &str) -> String {
    let path = current_dir().unwrap();
    let root = path.to_str().unwrap();
    format!("{}/{}/", root, dirname)
}

pub fn get_root_file(filename: &str) -> String {
    let path = current_dir().unwrap();
    let root = path.to_str().unwrap();
    format!("{}/{}", root, filename)
}

pub fn get_assets_dir() -> String {
    get_root_dir("assets")
}

pub fn get_asset_path(path: &str) -> String {
    format!("{}{}", get_assets_dir(), path)
}

pub fn get_file_name(path: &Path) -> String {
    path.file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        .split(".")
        .collect::<Vec<&str>>()
        .get(0)
        .unwrap()
        .to_string()
}

pub fn format_name(name: String) -> String {
    let s = name.replace("-", " ");
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().chain(c).collect(),
    }
}

pub fn find_files(path: &Path, extention: &str) -> Vec<String> {
    let mut paths = vec![];
    match fs::read_dir(path) {
        Ok(entries) => {
            if entries.count() == 0 {
                return paths;
            }
            let entries = fs::read_dir(path).unwrap();
            for entry in entries {
                let entry = entry.unwrap();

                if entry.path().is_dir() {
                    paths.extend(find_files(&entry.path(), extention))
                } else {
                    if extention.contains(".") {
                        if entry.path().ends_with(extention)
                            && !get_file_name(&entry.path()).starts_with("_")
                        {
                            paths.push(entry.path().to_string_lossy().to_string())
                        }
                    } else {
                        if entry.path().extension().unwrap() == extention
                            && !get_file_name(&entry.path()).starts_with("_")
                        {
                            paths.push(entry.path().to_string_lossy().to_string())
                        }
                    }
                }
            }
        }
        Err(_) => {
            // println!("This file or folder doesnt exist : {}", path.display());
        }
    }

    paths
}

pub fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

pub fn path_exist(path: &str) -> bool {
    Path::new(path).exists()
}
