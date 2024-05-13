use crate::xdg;
use std::path::PathBuf;
use xdgkit::desktop_entry::DesktopEntry;

use norm::fzf::{FzfParser, FzfV2};
use norm::Metric;

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

fn get_applications() -> Vec<Application> {
    let mut final_list = Vec::new();

    // add more sources here!
    final_list.extend(xdg::get_applications());

    return final_list;
}

pub struct Provider {
    applications: Vec<Application>
}

impl Provider {
    pub fn init() -> Self {
        return Self {
            applications: get_applications(),
        };
    }

    pub fn find(&self, query: &str) -> Vec<&Application> {
        let mut fzf = FzfV2::new();
        let mut parser = FzfParser::new();

        let query = parser.parse(query);

        let mut results = self.applications
            .iter()
            .filter_map(|entry| fzf.distance(query, &entry.name).map(|dist| (entry, dist)))
            .collect::<Vec<_>>();

        results.sort_by_key(|(_, dist)| *dist);

        return results.into_iter().map(|(entry, _)| entry).collect::<Vec<_>>()
    }
}


