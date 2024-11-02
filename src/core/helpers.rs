use ratatui::{style::Color, widgets::Block};
pub fn define_window_border_style<'a>(focused: bool) -> anyhow::Result<Block<'a>> {
    if focused {
        Ok(Block::bordered()
            .title_alignment(ratatui::layout::Alignment::Center)
            .style(Color::Blue))
    } else {
        Ok(Block::bordered()
            .title_alignment(ratatui::layout::Alignment::Center)
            .style(Color::Gray))
    }
}

pub fn logger(msg: String) {
    use std::fs::OpenOptions;
    use std::io::{self, Write};

    // Open the file in write mode (create if not exists).
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("runtime.log")
        .unwrap();

    writeln!(file, "{}", msg).unwrap();
}

pub fn clear_logger() {
    use std::fs::OpenOptions;
    use std::io::{self, Write};

    // Open the file in write mode (create if not exists).
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("runtime.log")
        .unwrap();

    writeln!(file, "{}", "").unwrap();
}
