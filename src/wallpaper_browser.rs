use std::fs::ReadDir;
use std::{fs, path::PathBuf};

use log::{info, debug, error, warn};

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
                info!("{} doesn't exists.", x.as_path().to_string_lossy().to_string());
                continue;
            }
        }
        self.wallpapers.clone().iter().map(|path| path.to_string_lossy().to_string()).collect()
    }
    
    fn iter_files(&mut self, iter: ReadDir) {
        for file in iter {
            if let Ok(wallpaper) = file {
                // if wallpaper.file_type().map_or(false, |file| file.is_file()) {
                    
                // }
                let wallpaper_path = PathBuf::from(wallpaper.file_name());
                if let Some(wallpaper_ext) = wallpaper_path.extension() {
                    let ext = &wallpaper_ext.to_string_lossy().to_string();
                    if ext == "jpg" || ext == "jpg" {
                        info!("Found a wallpaper: {}", wallpaper_path.display());
                        self.wallpapers.push(wallpaper_path);
                    }
                } else {
                    warn!("File '{}' doesn't have an extension", wallpaper_path.display());
                }
            }
        }
    }
}