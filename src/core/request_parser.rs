use std::fs;
use std::path::PathBuf;

use crate::components::structs::RequestStructure;

pub fn read_json_file(file_path: &PathBuf) -> serde_json::Result<RequestStructure> {
    let raw_json_data = fs::read_to_string(file_path).unwrap();

    let json_data = serde_json::from_str(&raw_json_data)?;

    Ok(json_data)
}

pub fn update_json_file(file_path: &PathBuf, data: RequestStructure) -> anyhow::Result<()> {
    let string_data = serde_json::to_string_pretty(&data)?;

    //let mut file = fs::File::open(file_path)?;
    fs::write(file_path, string_data)?;
    crate::core::helpers::logger(data);
    Ok(())
}
