use crate::core::enums::FocusedWindow;
use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, area: Rect, focus_window: &FocusedWindow) {
    let keybindings_text = Line::from(vec![
        Span::styled("Showing keybings for ", Style::default().fg(Color::Green)),
        Span::styled(
            focus_window.to_string(),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
    ]);
    let footer_text = match focus_window {
        FocusedWindow::Collections => keybindings_text,
        FocusedWindow::Request => keybindings_text,
        FocusedWindow::Response => keybindings_text,
        _ => Line::from(""),
    };

    frame.render_widget(
        Paragraph::new(footer_text).block(Block::new().padding(Padding::left(1))),
        area,
    );
}
