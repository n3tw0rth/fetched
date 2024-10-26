use color_eyre::Result;

mod constants;
mod core;
mod io;

use core::app;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = app::App::new().run(terminal);
    ratatui::restore();
    app_result
}
