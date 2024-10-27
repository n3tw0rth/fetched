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
