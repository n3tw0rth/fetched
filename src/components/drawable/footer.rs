use crate::core::enums::FocusedWindow;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Padding, Paragraph};
use ratatui::Frame;

pub fn draw(frame: &mut Frame, area: Rect, focus_window: &FocusedWindow) {
    let footer_text = match focus_window {
        FocusedWindow::Collections => "keybindings for collections",
        FocusedWindow::Request => "keybindings for request",
        FocusedWindow::Response => "keybindings for response",
        _ => "",
    };

    frame.render_widget(
        Paragraph::new(footer_text).block(Block::new().padding(Padding::left(1))),
        area,
    );
}
