use crate::components::structs::App;
use crate::core::enums::InputStrategy;
use crossterm::terminal;
use dirs::{config_dir, home_dir};
use std::fs::{self};
use std::io::prelude::*;

pub fn event_handler(input_strategy: InputStrategy, input: String, app: &mut App) {
    let cmds: Vec<_> = input.split(' ').collect();
    if input_strategy == InputStrategy::Command {
        match *cmds.get(0).unwrap() {
            "q" => exit_app(),
            _ => app.show_popup("Command not found".to_string()), //app.show_pop_up("Command not found".to_string()),
        }
    }
}

pub fn exit_app() {
    _ = terminal::disable_raw_mode();
    crate::core::helpers::clear_logger();
    ratatui::restore();
    std::process::exit(0x0100);
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
    }

    //setup the current working dir
    let cwd = std::env::current_dir();
    if !cwd.unwrap().exists() {
        // create the environment.toml file
        _ = fs::File::create(get_project_path().unwrap().join("environment.toml"))
    }
}

// collection contain lists of request data
pub fn list_collections() -> Vec<String> {
    fs::read_dir(std::env::current_dir().unwrap())
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
                .strip_prefix(std::env::current_dir().unwrap().to_str().unwrap())
                .unwrap()
                .to_string()
                .replace("/", "")
        })
        .collect()
}

pub fn list_collection_children(collection_name: String) -> Vec<String> {
    fs::read_dir(
        std::env::current_dir()
            .unwrap()
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
                std::env::current_dir()
                    .unwrap()
                    .join(collection_name.clone())
                    .to_str()
                    .unwrap(),
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
    _ = fs::create_dir(std::env::current_dir().unwrap().join(collection_name));
    Ok(())
}

pub fn create_request(
    collection_name: &String,
    children: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let file = fs::File::create(
        std::env::current_dir()
            .unwrap()
            .join(collection_name)
            .join(children),
    );
    let file_content = br#"{
  "method": "POST",
  "url": "https://example.com/api/resource",
  "headers": {
    "Content-Type": "application/json",
  },
  "query_parameters": {
    "search": "example",
    "page": "2"
  },
  "body_type": "json",
  "body": null,
  "options":{
    "validate_ssl": true,
    "follow_redirect": true,
    "attach_cookies": true,
    "proxy" : "",
    "timeout": 0.5
  },
  "metadata": {}
}
    "#;
    _ = file.unwrap().write_all(file_content);

    Ok(())
}

pub fn delete_collection(collection_name: String) -> Result<(), Box<dyn std::error::Error>> {
    _ = fs::remove_dir(std::env::current_dir().unwrap().join(collection_name)).unwrap();
    Ok(())
}

pub fn delete_collection_children(
    collection_name: String,
    children: String,
) -> Result<(), Box<dyn std::error::Error>> {
    _ = fs::remove_file(
        std::env::current_dir()
            .unwrap()
            .join(collection_name)
            .join(children),
    )
    .unwrap();
    Ok(())
}

pub fn get_file_path(
    collection: String,
    children: String,
) -> Result<String, Box<dyn std::error::Error>> {
    Ok(std::env::current_dir()?
        .join(collection)
        .join(children)
        .to_str()
        .unwrap()
        .to_string())
}
