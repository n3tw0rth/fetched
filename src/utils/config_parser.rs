use serde_derive::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ConfigFile {
    pub version: u8,
    pub env: Vec<ConfigFileENV>,
    pub trigger: String,
    pub stages: Vec<ConfigFileStages>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileENV {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileStages {
    pub stage: ConfigFileStage,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileStage {
    pub name: String,
    pub timeout: Option<u8>,
    pub steps: Option<Vec<String>>, // consider as a string and then do the selection based on the type shell,python or lua
    pub plugins: Option<Vec<ConfigFileStagePlugins>>,
}

#[derive(Debug, Deserialize)]
pub struct ConfigFileStagePlugins {
    pub name: String,
    pub when: String,
}

pub async fn parse(config_path: &str) -> Result<ConfigFile, Box<dyn std::error::Error>> {
    println!("Parsing the string");
    let config_file = tokio::fs::read_to_string(config_path).await?;

    let config: ConfigFile = serde_yaml::from_str(&config_file)?;
    Ok(config)
}
