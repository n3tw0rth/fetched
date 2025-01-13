use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub fn read_json_file(file_path: &PathBuf) -> anyhow::Result<Value> {
    let raw_json_data = fs::read_to_string(file_path)?;

    let json_data = serde_json::from_str(&raw_json_data)?;

    Ok(json_data)
}

pub fn update_json_file(file_path: &PathBuf, data: Value) -> anyhow::Result<()> {
    let string_data = serde_json::to_string_pretty(&data)?;

    //let mut file = fs::File::open(file_path)?;
    fs::write(file_path, string_data)?;

    crate::core::helpers::logger("updated the file");
    crate::core::helpers::logger(data);
    Ok(())
}
