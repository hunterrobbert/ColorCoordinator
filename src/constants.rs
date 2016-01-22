pub const THEME_DIR: &'static str = "src/schemes/";

// Dot file constants
pub const THEMED_DIR: &'static str = "dotfiles/themed/";
pub const UNTHEMED_DIR: &'static str = "dotfiles/unthemed/";

pub const XRES: &'static str = ".Xresources";
pub const I3CONFIG: &'static str = "config";
pub const I3STATUS: &'static str = "i3status.conf";
pub const VIM: &'static str = "coordinated.vim";
pub const DMENU: &'static str = "dmenu_run_themed";

pub const XRES_FINAL_DEST: &'static str = "/home/hunter/";
pub const I3CONFIG_FINAL_DEST: &'static str = "/home/hunter/.i3/";
pub const I3STATUS_FINAL_DEST: &'static str = "/home/hunter/.i3/";
pub const VIM_FINAL_DEST: &'static str = "/home/hunter/.vim/colors/";
pub const DMENU_FINAL_DEST: &'static str = "/home/hunter/.i3/";


pub fn get_supported_files() -> Vec<String> {
    vec![
        ".Xresources".to_string(),
        "i3 config".to_string(),
        "i3 status config".to_string(),
        "vim color theme".to_string(),
        "dmenu run script".to_string(),
        "firefox stylish css".to_string(),
    ]
}


// 
// pub fn get_unthemed_files() -> Vec<String> {
//     vec![
//         "dotfiles/unthemed/.Xresources".to_string(),
//         "dotfiles/unthemed/config".to_string(),
//         "dotfiles/unthemed/i3status.conf".to_string(),
//         "dotfiles/unthemed/coordinated.vim".to_string(),
//     ]
// }
