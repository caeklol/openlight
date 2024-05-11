use xdgkit::desktop_entry::{DesktopType, DesktopEntry};
use std::{env, fs};
use std::path::PathBuf;

use walkdir::WalkDir;

pub struct Application {
    title: String
}

// taken from fuzzel:
// https://codeberg.org/dnkl/fuzzel/src/branch/master/xdg.c
fn get_desktop_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    let data_home = env::var("XDG_DATA_HOME").ok();
    if data_home.is_some() {
        let data_home = PathBuf::from(data_home.unwrap());
        if data_home.exists() {
            dirs.push(data_home);
        }
    } else {
        let home_dir = env::var("HOME").expect("$HOME is not defined?");
        let share_dir = PathBuf::from(home_dir + "/.local/share");

        if share_dir.exists() {
            dirs.push(share_dir);
        }
    }

    let xdg_data_dirs = env::var("XDG_DATA_DIRS").ok();
    if xdg_data_dirs.is_some() {
        dirs.extend(
            xdg_data_dirs.unwrap()
            .split(":")
            .map(|s| PathBuf::from(s))
            .filter(|d| d.exists())
        );
    } else {
        let usr_local_share = PathBuf::from("/usr/local/share");
        let usr_share = PathBuf::from("/usr/share");

        if usr_share.exists() {
            dirs.push(usr_share);
        }

        if usr_local_share.exists() {
            dirs.push(usr_local_share);
        }
    }

    return dirs;
}

fn scan_desktop_files(dirs: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut files = Vec::new();
    for dir in dirs {
        let walker = WalkDir::new(dir).into_iter();
        for entry in walker {
            if let Ok(entry) = entry {
                if entry
                .path().extension()
                .and_then(|ext| Some(ext == "desktop"))
                .unwrap_or(false) {
                   files.push(entry.path().to_path_buf()); 
                }
            }
        }
    }

    return files;
}

fn read_desktop_files(files: Vec<PathBuf>) -> Vec<String> {
    let mut data = Vec::new();

    for path in files {
        if let Ok(file) = fs::read_to_string(path) {
            data.push(file);
        }
    }

    return data;
}

fn parse_files(files: Vec<String>) -> Vec<DesktopEntry> {
    return files
            .into_iter()
            .map(|f| DesktopEntry::read(f))
            .collect();
}

fn parse_desktop_entries(entries: Vec<DesktopEntry>) -> Vec<Application> {
    Vec::new()
}

pub fn get_applications() -> Vec<Application> {
    let dirs = get_desktop_dirs();
    let files = scan_desktop_files(dirs);
    let file_data = read_desktop_files(files);
    let desktop_entries = parse_files(file_data);
    let applications = parse_desktop_entries(desktop_entries);

    return applications;
}
