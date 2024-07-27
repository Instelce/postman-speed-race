use std::{
    env::current_dir,
    fs::{self, read_to_string},
    io,
    path::Path,
};

pub fn get_root_dir(dirname: &str) -> String {
    #[cfg(not(target_family = "wasm"))]
    let path = current_dir().unwrap();
    #[cfg(target_family = "wasm")]
    let path = Path::new("/");
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
    #[cfg(not(target_family = "wasm"))]
    {
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

    #[cfg(target_family = "wasm")]
    {
        // Add manualy all files in the assets folder
        // yes it's not a joke
        let all_paths = vec![
            "images/postman.ase".to_string(),
            "images/ui/letter.ase".to_string(),
            "images/ui/big-button.ase".to_string(),
            "images/ui/button.ase".to_string(),
            "images/ui/star.ase".to_string(),
            "images/postman.gif".to_string(),
            "images/tiles/flowers.png".to_string(),
            "images/tiles/tiles.ase".to_string(),
            "images/tiles/tiles.png".to_string(),
            "images/tiles/trees.ase".to_string(),
            "images/tiles/flowers.ase".to_string(),
            "images/objects/orangebull-can.ase".to_string(),
            "images/obstacles/work-cone.ase".to_string(),
            "images/obstacles/manhole-cover.ase".to_string(),
            "images/obstacles/water-puddle.ase".to_string(),
            "images/obstacles/road-work.ase".to_string(),
            "images/splash.png".to_string(),
            "images/with_bevy.png".to_string(),
            "images/letter-box.ase".to_string(),
            "images/houses/5.png".to_string(),
            "images/houses/3.ase".to_string(),
            "images/houses/2.ase".to_string(),
            "images/houses/4.png".to_string(),
            "images/houses/6.ase".to_string(),
            "images/houses/2.png".to_string(),
            "images/houses/5.ase".to_string(),
            "images/houses/4.ase".to_string(),
            "images/houses/1.png".to_string(),
            "images/houses/1.ase".to_string(),
            "images/houses/3.png".to_string(),
            "images/houses/6.png".to_string(),
            "images/bevy.png".to_string(),
            "particles/player.particle.ron".to_string(),
            "particles/_example.particle.ron".to_string(),
            "sketch.ase".to_string(),
            "audio/soundtracks/Go.ogg".to_string(),
            "audio/soundtracks/RUNAWAY.ogg".to_string(),
            "audio/soundtracks/ChillMenu.ogg".to_string(),
            "maps/chunks.ldtk".to_string(),
            "maps/maps.ldtk".to_string(),
            "maps/road.png".to_string(),
            "maps/road.ase".to_string(),
            "icons/icon.png".to_string(),
            "fonts/minecraft.ttf".to_string(),
            "fonts/minecraftia.ttf".to_string(),
            "fonts/gamer.ttf".to_string(),
            "fonts/pixeled.ttf".to_string(),
            "fonts/pixelify_sans.ttf".to_string(),
            "upload/banner.ase".to_string(),
            "upload/postman.png".to_string(),
            "upload/bg.ase".to_string(),
            "upload/letter.png".to_string(),
            "upload/bg.png".to_string(),
            "upload/banner.png".to_string(),
            "audio/sfx/button_hovered.ogg".to_string(),
            "audio/sfx/button_pressed.ogg".to_string(),
            "audio/sfx/launch.ogg".to_string(),
        ];

        // filter all paths with the extention and the path parameter
        let mut paths = vec![];

        for path in &all_paths {
            if path.contains(extention) && path.contains(path) {
                paths.push(path.clone());
            }
        }

        paths
    }
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

pub fn show_all_files(path: &Path) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            show_all_files(&path)?;
        } else {
            println!(
                "{}",
                path.display()
                    .to_string()
                    .split("race/")
                    .collect::<Vec<&str>>()[1]
            );
        }
    }
    Ok(())
}

#[test]
fn test_show_all() {
    show_all_files(Path::new(&get_assets_dir()));
}
