use ratatui::widgets::{Block, Borders, Paragraph};

pub fn render<'a>() -> Result<Paragraph<'a>, Box<dyn std::error::Error>> {
    Ok(Paragraph::new("Header").block(Block::bordered().borders(Borders::TOP)))
}
