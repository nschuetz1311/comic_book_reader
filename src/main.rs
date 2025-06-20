use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, ListBoxRow, ScrolledWindow, Label};
use std::path::PathBuf;
use walkdir::WalkDir;
use std::collections::HashMap;

fn main() {
    let app = Application::builder()
        .application_id("com.example.TestNiko123")
        .build();

    app.connect_activate(build_ui);

    app.run();
}

fn build_ui(app: &Application) {
    let comics_folder_path = dirs::home_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("Comics");

    let cbr_files = find_cbr_files(&comics_folder_path);

    let list_box = ListBox::new();
 
    if cbr_files.is_empty() {
        let row = ListBoxRow::new();
        let label = Label::new(Some("No CBR Files found"));
        row.set_child(Some(&label));
        list_box.append(&row);
    } else {
        for folder_name in cbr_files {
            let row = ListBoxRow::new();
            let label = Label::new(Some(&folder_name.0));
                label.set_xalign(0.0);
                row.set_child(Some(&label));
                list_box.append(&row);
        }
    }

    let scrolled_window = ScrolledWindow::builder()
        .min_content_width(800)
        .min_content_height(600)
        .child(&list_box)
        .build();

    let window = ApplicationWindow::builder()
        .application(app)
        .title("CBR Reader")
        .child(&scrolled_window)
        .build();

    window.show();
}


fn find_cbr_files(path: &PathBuf) -> HashMap<String, PathBuf> {
    let mut array = HashMap::new();

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let entry_path = entry.path();
        if entry.file_type().is_file() {
            if let Some(ext) = entry_path.extension() {
                if ext.to_string_lossy().to_ascii_lowercase() == "cbr" {
                    match entry_path.strip_prefix(path) {
                        Ok(rel_path) => {
                            // Get the first component of the relative path
                            if let Some(first_component) = rel_path.components().next() {
                                // save the folder_name and its path into a HashMap which then
                                // again can be used to read the next contents
                                let folder_name = first_component.as_os_str().to_string_lossy().to_string();
                                array.entry(folder_name.clone()).or_insert(path.join(&folder_name));
                            }
                        }
                        Err(e) => eprintln!("Error stripping prefix: {}", e),
                    }
                }
            }
        }
    }

    array
}
