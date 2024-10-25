use std::fs;
pub fn create_file(name: String) {
    _ = fs::File::create(name);
}
