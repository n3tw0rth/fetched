use crate::utils::config_parser;

pub fn async_runner() -> Result<(), Box<dyn std::error::Error>> {
    let config = config_parser::parse(&"./examples/script.yaml")?;
    for env in config.env.iter() {
        println!("{:#?}", env);
    }
    Ok(())
}
