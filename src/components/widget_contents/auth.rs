use ratatui::widgets::{Block, BorderType, Borders, Paragraph};

pub fn render<'a>() -> Result<(), Box<dyn std::error::Error>> {
    Paragraph::new("Authentication").block(
        Block::bordered()
            .border_type(BorderType::Rounded)
            .borders(Borders::TOP),
    );

    Ok(())
}
