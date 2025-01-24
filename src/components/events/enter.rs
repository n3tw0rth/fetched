use crate::core::request_parser;
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn request_widget_edit_headers_enter_event(
    input_buffer: &HashMap<u8, String>,
    file_path: &PathBuf,
) -> Result<()> {
    let mut data = request_parser::read_json_file(&file_path)?;

    // Update an existing field inside the nested object
    data.headers.insert(
        input_buffer.get(&0).unwrap().to_string(),
        //json!(input_buffer.get(&1).unwrap()),
        input_buffer.get(&1).unwrap().to_string(),
    );

    crate::core::helpers::logger("called");

    // TODO:
    // update the json and  overwrite the file
    let _ = request_parser::update_json_file(&file_path, data)?;

    Ok(())
}
