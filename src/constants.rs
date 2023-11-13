use dirs::home_dir;
use std::path::PathBuf;


pub fn user_home() -> PathBuf {
    home_dir().unwrap()
}

pub fn directories(user_home: PathBuf) -> Vec<PathBuf> { 
    vec![
        PathBuf::from("/usr/share/backgrounds"),
        PathBuf::from("/usr/share/wallpapers"),
        user_home.clone().join("Pictures"),
    ]
}