use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Label, ListBox, ListBoxRow, ScrolledWindow};
use serde::Deserialize;
use std::{
    collections::HashMap,
    env, fs,
    io::{self, Write},
    path::PathBuf,
};
use walkdir::WalkDir;

const M_CONFIG_NAME: &str = "cbr-config.json";

#[derive(Deserialize, Debug)]
struct Config {
    comics_folder_path: PathBuf,
}

// initial implementation for missing config
// in the future this should lead to a prompt letting the user choose a location
impl Default for Config {
    fn default() -> Self {
        Self {
            comics_folder_path: PathBuf::from("Comics"),
        }
    }
}

fn read_config() -> Option<Config> {
    let current_dir = env::current_dir().ok()?;
    let config_path = current_dir.join(M_CONFIG_NAME);
    let contents = fs::read_to_string(config_path).ok()?;
    serde_json::from_str(&contents).ok()
}

fn main() {
    let app = Application::builder()
        .application_id("com.example.TestNiko123")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn find_config() -> bool {
    let Ok(path) = env::current_dir() else {
        eprintln!("Failed to get current directory");
        return false;
    };

    // println!("Current working directory: {}", path.display());

    let Ok(entries) = fs::read_dir(&path) else {
        eprintln!("Failed to read directory contents!");
        return false;
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        if file_name == M_CONFIG_NAME {
            return true;
        }
    }

    false
}

fn create_config(buf: &[u8]) -> io::Result<()> {
    let mut new_file: fs::File = fs::File::create(M_CONFIG_NAME)?;
    let _ = new_file.write(buf);
    Ok(())
}

fn build_ui(app: &Application) {
    if !find_config() {
        // next step will be adding a prompt
        // to create and fill out the "M_CONFIG_NAME"
        let buf: &[u8] = br#"{
    "comics_folder_path": "/home/niko/Comics"
}"#;
        if let Err(e) = create_config(buf) {
            eprintln!("Failed to create config file: {}", e);
        }
    }

    let config = read_config().unwrap_or_default();
    let comics_folder_path = config.comics_folder_path;

    let cbr_files = find_cbr_files(&comics_folder_path);

    let list_box = ListBox::new();

    if cbr_files.is_empty() {
        let row = ListBoxRow::new();
        let label = Label::new(Some("No CBR Files found"));
        row.set_child(Some(&label));
        list_box.append(&row);
    } else {
        for path in cbr_files {
            let row = ListBoxRow::new();
            let label = Label::new(Some(&path.display().to_string()));
            label.set_xalign(0.0);
            row.set_child(Some(&label));
            list_box.append(&row);
        }
    }

    display_ui_stuff(app, &list_box);
}

fn display_ui_stuff(app: &Application, list_box: &ListBox) {
    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(800)
        .min_content_height(600)
        .child(list_box)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("CBR Reader")
        .child(&scrolled_window)
        .build();

    window.show();
}

fn find_cbr_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut results: Vec<PathBuf> = Vec::new();
    let mut hm_array: HashMap<String, PathBuf> = HashMap::new();

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let entry_path = entry.path();

        if !entry.file_type().is_file() {
            continue;
        }

        let Some(ext) = entry_path.extension() else {
            continue;
        };

        if ext.to_string_lossy().to_ascii_lowercase() != "cbr" {
            continue;
        }

        let Ok(rel_path) = entry_path.strip_prefix(path) else {
            eprintln!("Error stripping prefix from {}", entry_path.display());
            continue;
        };

        if let Some(first_component) = rel_path.components().next() {
            let folder_name = first_component.as_os_str().to_string_lossy().to_string();
            hm_array
                .entry(folder_name.clone())
                .or_insert(path.join(&folder_name));
        }

        results.push(entry_path.to_path_buf());
    }
    println!("{:?}", hm_array.keys());

    results
}
