use crate::core::enums::InputStrategy;
use crossterm::terminal;
use dirs::{config_dir, home_dir};
use ratatui::widgets::ListItem;
use std::fmt;
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
            "q" => {
                _ = std::process::Command::new("clear").status();
                _ = terminal::disable_raw_mode();
                crate::core::helpers::clear_logger();
                std::process::exit(0x0100);
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
pub fn list_collections() -> Vec<String> {
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
            //"\u{f024b} {}",
            entry
                .1
                .display()
                .to_string()
                .strip_prefix(&Constants::new().app_config_path.display().to_string())
                .unwrap()
                .to_string()
                .replace("/", "")
        })
        .collect()
}

pub fn list_collection_children(collection_name: String) -> Vec<String> {
    fs::read_dir(
        Constants::new()
            .app_config_path
            .join(collection_name.clone()),
    )
    .unwrap()
    .filter_map(|entry| {
        let entry = entry.ok()?; // Handle errors with filter_map
        let metadata = entry.metadata().ok()?;
        if metadata.is_file() {
            Some(entry.path()) // Collect path if it's a file
        } else {
            None
        }
    })
    .enumerate()
    .map(|entry| {
        entry
            .1
            .display()
            .to_string()
            .strip_prefix(
                &Constants::new()
                    .app_config_path
                    .join(collection_name.clone())
                    .display()
                    .to_string(),
            )
            .unwrap()
            .to_string()
            .replace("/", "")
    })
    .collect()
}

pub fn get_project_path() -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    Ok(config_dir().expect("dir does not exists").join("fetched"))
}

pub fn create_collection(collection_name: String) -> Result<(), Box<dyn std::error::Error>> {
    _ = fs::create_dir(get_project_path().unwrap().join(collection_name));
    Ok(())
}

pub fn create_collection_children(
    collection_name: String,
    children: String,
) -> Result<(), Box<dyn std::error::Error>> {
    _ = fs::File::create(
        get_project_path()
            .unwrap()
            .join(collection_name)
            .join(children),
    );
    Ok(())
}

pub fn delete_collection(collection_name: String) -> Result<(), Box<dyn std::error::Error>> {
    super::helpers::logger(collection_name.clone());
    _ = fs::remove_dir(get_project_path().unwrap().join(collection_name)).unwrap();
    Ok(())
}

pub fn delete_collection_children(
    collection_name: String,
    children: String,
) -> Result<(), Box<dyn std::error::Error>> {
    super::helpers::logger(
        get_project_path()
            .unwrap()
            .join(collection_name.clone())
            .join(children.clone())
            .display()
            .to_string(),
    );
    _ = fs::remove_file(
        get_project_path()
            .unwrap()
            .join(collection_name)
            .join(children),
    )
    .unwrap();
    Ok(())
}

pub fn open_file(collection: String, children: String) {
    todo!()
    //_ = std::process::Command::new("nvim")
    //    .arg(get_project_path().unwrap().join(collection).join(children))
    //    .spawn();
}
