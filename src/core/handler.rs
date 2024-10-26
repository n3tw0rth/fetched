use crate::core::enums::InputStrategy;
use std::fs::File;

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
