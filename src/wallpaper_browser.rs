use std::fs::ReadDir;
use std::{fs, path::PathBuf};

use log::{error, info, warn};

pub struct WallpaperBrowser {
    wallpapers: Vec<PathBuf>,
}

impl WallpaperBrowser {
    pub fn new() -> Self {
        info!("Creating WallpaperBrowser...");
        WallpaperBrowser { wallpapers: vec![] }
    }
    pub fn browse(&mut self, directories: Vec<PathBuf>) -> Vec<String> {
        info!("Browsing wallpapers...");
        for x in directories {
            if x.exists() {
                if let Ok(iterator) = fs::read_dir(x) {
                    self.iter_files(iterator);
                }
            } else {
                info!(
                    "{} doesn't exists.",
                    x.as_path().to_string_lossy().to_string()
                );
                continue;
            }
        }
        self.wallpapers
            .clone()
            .iter()
            .map(|path| path.to_string_lossy().to_string())
            .collect()
    }

    fn iter_files(&mut self, iter: ReadDir) {
        for file in iter {
            match file {
                Ok(wallpaper) => {
                    // if wallpaper.file_type().map_or(false, |file| file.is_file()) {

                    // }
                    let wallpaper_name = PathBuf::from(wallpaper.file_name());
                    let wallpaper_path = wallpaper.path();

                    if let Some(wallpaper_ext) = wallpaper_name.extension() {
                        let ext = &wallpaper_ext.to_string_lossy().to_string();
                        if ext == "jpg" || ext == "png" {
                            info!("Found a wallpaper: {}", wallpaper_name.display());
                            self.wallpapers.push(wallpaper_path);
                        }
                    } else {
                        warn!(
                            "File '{}' doesn't have an extension",
                            wallpaper_name.display()
                        );
                    }
                }
                Err(err) => {
                    error!("Error while browsing wallpapers: {}", err);
                    info!("Ignoring this wallpaper...");
                }
            }
        }
    }
}
