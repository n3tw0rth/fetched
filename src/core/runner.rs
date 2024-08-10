use crate::utils::config_parser;

pub async fn async_runner() -> Result<(), Box<dyn std::error::Error>> {
    let config = config_parser::parse(&"./examples/script.yaml").await?;
    for env in config.env.iter() {
        println!("{:#?}", env);
    }
    Ok(())
}
