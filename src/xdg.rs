use crate::provider::Application;
use std::path::PathBuf;
use std::{env, fs};
use xdgkit::desktop_entry::DesktopEntry;

use walkdir::WalkDir;

// taken from fuzzel:
// https://codeberg.org/dnkl/fuzzel/src/branch/master/xdg.c
fn get_desktop_dirs() -> Vec<PathBuf> {
    let mut dirs = Vec::new();

    if let Ok(data_home) = env::var("XDG_DATA_HOME") {
        let data_home = PathBuf::from(data_home);
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

    if let Ok(xdg_data_dirs) = env::var("XDG_DATA_DIRS") {
        dirs.extend(
            xdg_data_dirs
                .split(":")
                .map(|s| PathBuf::from(s))
                .filter(|d| d.exists()),
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
                    .path()
                    .extension()
                    .and_then(|ext| Some(ext == "desktop"))
                    .unwrap_or(false)
                {
                    files.push(entry.path().to_path_buf());
                }
            }
        }
    }

    return files;
}

fn read_desktop_files(files: Vec<PathBuf>) -> Vec<String> {
    return files
        .into_iter()
        .filter_map(|path| fs::read_to_string(path).ok())
        .collect();
}

fn parse_desktop_entries(data: Vec<String>) -> Vec<DesktopEntry> {
    return data.into_iter().map(|f| DesktopEntry::read(f)).collect();
}

fn generate_applications(entries: Vec<DesktopEntry>) -> Vec<Application> {
    return entries
        .into_iter()
        .filter_map(|e| Application::try_from(e).ok())
        .collect();
}

pub fn get_applications() -> Vec<Application> {
    let dirs = get_desktop_dirs();
    let files = scan_desktop_files(dirs);
    let file_data = read_desktop_files(files);
    let desktop_entries = parse_desktop_entries(file_data);
    let applications = generate_applications(desktop_entries);

    return applications;
}
