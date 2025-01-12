use color_eyre::Result;

mod components;
mod constants;
mod core;

use components::structs::App;

fn main() -> Result<()> {
    core::handler::create_config_folder();
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::new().run(terminal);
    ratatui::restore();
    app_result
}
