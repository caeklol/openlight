use crate::xdg;
use std::path::PathBuf;
use xdgkit::desktop_entry::DesktopEntry;

#[derive(Debug)]
pub struct Application {
    name: String,
    exec: String,
    exec_path: Option<PathBuf>,
    desc: Option<String>,
    class: Option<String>,
}

impl TryFrom<DesktopEntry> for Application {
    type Error = &'static str;

    fn try_from(value: DesktopEntry) -> Result<Self, Self::Error> {
        // let path = value.path.and_then(|p| PathBuf::from(&p).exists().then(|| PathBuf::from(&p))); <-- waytoodank
        let mut path = None;

        if let Some(p) = value.path {
            let path_buf = PathBuf::from(&p);
            if path_buf.exists() {
                path = Some(path_buf);
            }
        }

        return Ok(Application {
            name: value.name.ok_or("No name")?,
            exec: value.exec.ok_or("No exec")?,
            exec_path: path,
            desc: value.comment,
            class: value.generic_name,
        });
    }
}

pub fn get_applications() -> Vec<Application> {
    let mut final_list = Vec::new();

    // add more sources here!
    final_list.extend(xdg::get_applications());

    return final_list;
}
