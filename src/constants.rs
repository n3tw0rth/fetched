pub struct Constants {
    pub file_extension: String,
}

impl Constants {
    pub fn new() -> Self {
        Self {
            file_extension: ".ft".to_string(),
        }
    }
}

