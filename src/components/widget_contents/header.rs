use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn render<'a>() -> Result<Paragraph<'a>, Box<dyn std::error::Error>> {
    Ok(Paragraph::new("Headers").block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .borders(Borders::TOP),
    ))
}
