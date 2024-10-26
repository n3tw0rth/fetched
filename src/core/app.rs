use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, List, ListItem, Paragraph},
    DefaultTerminal, Frame,
};

use crate::core::enums::{InputMode, InputStrategy};
use crate::core::handler::edit_event_handler;
use crate::io::file_create::create_file;

//App holds the state of the application
pub struct App {
    // Current value of the input box
    input: String,
    // Position of cursor in the editor area.
    character_index: usize,
    // Current input mode
    input_mode: InputMode,
    // Input strategy
    input_strategy: InputStrategy,
    // History of recorded messages
    messages: Vec<String>,
}

impl App {
    pub const fn new() -> Self {
        Self {
            input: String::new(),
            input_mode: InputMode::Normal,
            input_strategy: InputStrategy::Command,
            messages: Vec::new(),
            character_index: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.character_index.saturating_sub(1);
        self.character_index = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.character_index.saturating_add(1);
        self.character_index = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        let index = self.byte_index();
        self.input.insert(index, new_char);
        self.move_cursor_right();
    }

    /// Returns the byte index based on the character position.
    ///
    /// Since each character in a string can be contain multiple bytes, it's necessary to calculate
    /// the byte index based on the index of the character.
    fn byte_index(&self) -> usize {
        self.input
            .char_indices()
            .map(|(i, _)| i)
            .nth(self.character_index)
            .unwrap_or(self.input.len())
    }

    fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.character_index != 0;
        if is_not_cursor_leftmost {
            // Method "remove" is not used on the saved text for deleting the selected char.
            // Reason: Using remove on String works on bytes instead of the chars.
            // Using remove would require special care because of char boundaries.

            let current_index = self.character_index;
            let from_left_to_current_index = current_index - 1;

            // Getting all characters before the selected character.
            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            // Getting all characters after selected character.
            let after_char_to_delete = self.input.chars().skip(current_index);

            // Put all characters together except the selected one.
            // By leaving the selected one out, it is forgotten and therefore deleted.
            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn submit_message(&mut self) {
        edit_event_handler(self.input_strategy.clone(), self.input.clone());
        self.messages.push(self.input.clone());
        self.input.clear();
        self.reset_cursor();
        self.input_mode = InputMode::Normal;
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        //KeyCode::Char('e') => {
                        //    self.input_mode = InputMode::Editing;
                        //}
                        KeyCode::Char('q') => {
                            return Ok(());
                        }
                        KeyCode::Char(':') => {
                            self.input_strategy = InputStrategy::Command;
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('/') => {
                            self.input_strategy = InputStrategy::Search;
                            self.input_mode = InputMode::Editing;
                        }
                        _ => {}
                    },
                    InputMode::Editing if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => self.input_mode = InputMode::Normal,
                        _ => {}
                    },
                    InputMode::Editing => {}
                }
            }
        }
    }

    fn draw(&self, frame: &mut Frame) {
        let mut vertical_layout: [Rect; 3] = Layout::vertical([
            Constraint::Length(0),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .areas(frame.area());
        if self.input_mode == InputMode::Editing {
            vertical_layout = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .areas(frame.area());
        }
        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Editing => Style::default().fg(Color::Yellow),
            })
            .block(
                Block::bordered().title(if self.input_strategy == InputStrategy::Command {
                    "Command"
                } else {
                    "Search"
                }),
            );
        // render the input field only in editing mode
        if self.input_mode == InputMode::Editing {
            frame.render_widget(input, *vertical_layout.get(0).unwrap());
        }
        match self.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Editing => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                vertical_layout.get(0).unwrap().x + self.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                vertical_layout.get(0).unwrap().y + 1,
            )),
        }

        // url widget
        let horizontal_layout: [Rect; 2] =
            Layout::horizontal([Constraint::Length(8), Constraint::Min(1)])
                .areas(*vertical_layout.get(1).unwrap());
        // http method widget
        let http_method_widget = Paragraph::new("POST")
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::bordered().style(Color::Yellow));
        frame.render_widget(http_method_widget, *horizontal_layout.get(0).unwrap());
        // url
        let url_widget = Paragraph::new("https://somewhere.com/api/v1/users")
            .block(Block::bordered().style(Color::White));
        frame.render_widget(url_widget, *horizontal_layout.get(1).unwrap());

        let messages: Vec<ListItem> = self
            .messages
            .iter()
            .enumerate()
            .map(|(i, m)| {
                let content = Line::from(Span::raw(format!("{i}: {m}")));
                ListItem::new(content)
            })
            .collect();
        let messages = List::new(messages).block(
            Block::bordered()
                .title("Messages")
                .title_alignment(ratatui::layout::Alignment::Center),
        );
        frame.render_widget(messages, *vertical_layout.get(2).unwrap());
    }
}
