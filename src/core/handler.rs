use crate::core::enums::InputStrategy;
use dirs::home_dir;
use ratatui::widgets::ListItem;
use std::fs::File;
use std::fs::{self};

use crate::constants::Constants;

pub fn edit_event_handler(input_strategy: InputStrategy, input: String) {
    let file_format = Constants::new().file_extension;
    let cmds: Vec<_> = input.split(' ').collect();
    if input_strategy == InputStrategy::Command {
        match *cmds.get(0).unwrap() {
            "new" => {
                let mut filename = cmds.get(1).unwrap().to_string();
                filename = format!("{filename}{file_format}").to_string();
                println!("{0}", filename);
                _ = File::create(filename);
            }
            _ => {}
        }
    }
}

pub fn create_config_folder() {
    // Get the path to ~/.config/fetched/
    let config_dir = home_dir()
        .expect("Could not retrieve home directory")
        .join(".config")
        .join("fetched");

    // Check if the directory exists
    if !config_dir.exists() {
        // Create the directory (and any necessary parent directories)
        fs::create_dir_all(&config_dir).unwrap();
        println!("Created directory: {:?}", config_dir);
    } else {
        println!("Directory already exists: {:?}", config_dir);
    }
}

// collection contain lists of request data
pub fn list_collections<'a>() -> Vec<ListItem<'a>> {
    fs::read_dir(Constants::new().app_config_path)
        .unwrap()
        .filter_map(|entry| {
            let entry = entry.ok()?; // Handle errors with filter_map
            let metadata = entry.metadata().ok()?;
            if !metadata.is_file() {
                Some(entry.path()) // Collect path if it's a file
            } else {
                None
            }
        })
        .enumerate()
        .map(|entry| {
            ListItem::new(format!(
                "\u{f024b} {}",
                entry
                    .1
                    .display()
                    .to_string()
                    .replace(&Constants::new().app_config_path.display().to_string(), "")
                    .replace("/", "")
            ))
        })
        .collect()
}
