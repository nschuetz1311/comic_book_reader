use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, ListBox, ListBoxRow, ScrolledWindow, Label};
use std::path::PathBuf;
use walkdir::WalkDir;

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
        for path in cbr_files {
        let row = ListBoxRow::new();
        let label = Label::new(Some(&path.display().to_string()));
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


fn find_cbr_files(path: &PathBuf) -> Vec<PathBuf> {
    let mut results = Vec::new();

    for entry in WalkDir::new(path)
        .follow_links(false)
        .into_iter()
        .filter_map(Result::ok)
    {
        let entry_path = entry.path();

        if let Ok(metadata) = entry.metadata() {
            if metadata.is_file() {
                if let Some(ext) = entry_path.extension() {
                    if ext.to_string_lossy().to_ascii_lowercase() == "cbr" {
                        results.push(entry_path.to_path_buf());
                    }
                }
            }
        }
    }

    results
}
