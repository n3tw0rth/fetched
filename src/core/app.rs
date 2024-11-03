use crate::components::manager;
use crate::components::structs::App;
use crate::core::enums::{
    FocusedWindow, InputMode, InputStrategy, RequestWidgetTabs, ResponseWidgetTabs, ThemeState,
    WidgetType, WindowMotion, WindowOperation,
};
use crate::core::handler;
use crate::core::theme;
use color_eyre::Result;
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListState, Paragraph, Tabs},
    DefaultTerminal, Frame,
};
use std::error::Error;
use strum::IntoEnumIterator;

impl App {
    pub fn new() -> Self {
        Self {
            theme: theme::get_theme().unwrap(),
            input: String::new(),
            input_mode: InputMode::Normal,
            input_strategy: InputStrategy::Command,
            character_index: 0,
            focused_window: FocusedWindow::Collections,
            //state
            collections: handler::list_collections(),
            collection_window_list_state: ListState::default().with_selected(Some(1)),
            selected_collection: "".to_string(),
            show_collection_children: false,
            // request tabs
            selected_tab: 0,
            //response tabs
            selected_response_tab: 0,
            current_operation: WindowOperation::Null,
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

    fn prompt(&mut self, operation: WindowOperation) {
        self.input_mode = InputMode::Editing;
        self.input_strategy = InputStrategy::Prompt;

        self.current_operation = operation;
    }

    fn execute_operation_on_selected_window(&mut self, operation: WindowOperation, promt: String) {
        match self.focused_window {
            FocusedWindow::Collections => match operation {
                WindowOperation::CollectionCreate => {
                    if self.show_collection_children {
                        handler::create_collection_children(self.selected_collection.clone(), promt)
                            .unwrap()
                    } else {
                        handler::create_collection(promt).unwrap();
                    }
                }
                WindowOperation::CollectionDelete => {
                    if promt == "y" {
                        if self.show_collection_children {
                            handler::delete_collection_children(
                                self.selected_collection.clone(),
                                self.collections
                                    .get(self.collection_window_list_state.selected().unwrap())
                                    .unwrap()
                                    .to_string(),
                            )
                            .unwrap();
                        } else {
                            _ = handler::delete_collection(
                                self.collections
                                    .get(self.collection_window_list_state.selected().unwrap())
                                    .unwrap()
                                    .to_string(),
                            );
                        }
                    }
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
        if self.show_collection_children {
            self.collections = handler::list_collection_children(self.selected_collection.clone())
        } else {
            self.collections = handler::list_collections()
        }
    }

    fn select_collection_to_send_motion(&mut self, motion: WindowMotion) {
        match self.focused_window {
            FocusedWindow::Collections => match motion {
                WindowMotion::Up => {
                    self.collection_window_list_state.select_next();
                }
                WindowMotion::Down => {
                    self.collection_window_list_state.select_previous();
                }
                WindowMotion::Left => {
                    if self.show_collection_children {
                        self.show_collection_children = false;
                        self.selected_collection = "".to_string();
                        self.collections = super::handler::list_collections()
                    }
                }
                WindowMotion::Right => {
                    if !self.show_collection_children {
                        self.show_collection_children = true;
                        self.selected_collection = self
                            .collections
                            .get(self.collection_window_list_state.selected().unwrap())
                            .unwrap()
                            .to_string();

                        self.collections = super::handler::list_collection_children(
                            self.selected_collection.clone(),
                        )
                    }
                }
                _ => {}
            },
            FocusedWindow::Request => match motion {
                WindowMotion::Left => {
                    if self.selected_tab == 0 {
                        self.selected_tab = RequestWidgetTabs::iter().count() - 1;
                    } else {
                        self.selected_tab = self.selected_tab - 1;
                    };
                }
                WindowMotion::Right => {
                    if self.selected_tab == RequestWidgetTabs::iter().count() - 1 {
                        self.selected_tab = 0;
                    } else {
                        self.selected_tab = self.selected_tab + 1;
                    };
                }
                _ => {}
            },
            FocusedWindow::Response => match motion {
                WindowMotion::Left => {
                    if self.selected_response_tab == 0 {
                        self.selected_response_tab = ResponseWidgetTabs::iter().count() - 1;
                    } else {
                        self.selected_response_tab = self.selected_response_tab - 1;
                    };
                }
                WindowMotion::Right => {
                    if self.selected_response_tab == ResponseWidgetTabs::iter().count() - 1 {
                        self.selected_response_tab = 0;
                    } else {
                        self.selected_response_tab = self.selected_response_tab + 1;
                    };
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn submit_message(&mut self) {
        if self.input_strategy == InputStrategy::Prompt {
            self.execute_operation_on_selected_window(self.current_operation, self.input.clone());
        } else {
            handler::edit_event_handler(self.input_strategy.clone(), self.input.clone());
        }
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
                        KeyCode::Char(':') => {
                            self.input_strategy = InputStrategy::Command;
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('/') => {
                            self.input_strategy = InputStrategy::Search;
                            self.input_mode = InputMode::Editing;
                        }
                        KeyCode::Char('1') => {
                            self.focused_window = FocusedWindow::Collections;
                        }
                        KeyCode::Char('2') => {
                            self.focused_window = FocusedWindow::Request;
                        }
                        KeyCode::Char('3') => {
                            self.focused_window = FocusedWindow::Response;
                        }
                        KeyCode::Char('k') => {
                            self.select_collection_to_send_motion(WindowMotion::Down)
                        }
                        KeyCode::Char('j') => {
                            self.select_collection_to_send_motion(WindowMotion::Up)
                        }
                        KeyCode::Char('h') => {
                            self.select_collection_to_send_motion(WindowMotion::Left)
                        }
                        KeyCode::Char('l') => {
                            self.select_collection_to_send_motion(WindowMotion::Right)
                        }
                        KeyCode::Char('a') => self.prompt(WindowOperation::CollectionCreate),
                        KeyCode::Char('d') => self.prompt(WindowOperation::CollectionDelete),
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

    fn decide_input_title(&self) -> Result<String, Box<dyn Error>> {
        if self.input_strategy == InputStrategy::Command {
            Ok("Command".to_string())
        } else if self.input_strategy == InputStrategy::Search {
            Ok("Search".to_string())
        } else {
            match self.current_operation {
                WindowOperation::CollectionCreate => {
                    if self.show_collection_children {
                        Ok("Request Name".to_string())
                    } else {
                        Ok("Collection Name".to_string())
                    }
                }
                WindowOperation::CollectionDelete => Ok("Delete Collection [y/N]".to_string()),
                _ => todo!(),
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
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
                InputMode::Editing => {
                    Style::default().fg(Color::from_u32(self.theme.focus.foreground))
                }
            })
            .block(Block::bordered().title(self.decide_input_title().unwrap()));
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

        // 1st horizontal layout
        // split the 2nd vertical layout horizontally
        let horizontal_layout: [Rect; 2] =
            Layout::horizontal([Constraint::Length(8), Constraint::Min(1)])
                .areas(*vertical_layout.get(1).unwrap());
        // http method widget
        let http_method_widget = Paragraph::new("POST")
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(Block::bordered().style(Color::from_u32(self.theme.focus.border)));
        frame.render_widget(http_method_widget, *horizontal_layout.get(0).unwrap());
        // url
        let url_widget = Paragraph::new("https://somewhere.com/api/v1/users")
            .block(Block::bordered().style(Color::White));
        frame.render_widget(url_widget, *horizontal_layout.get(1).unwrap());

        // 2st horizontal layout
        // split the 3nd vertical layout horizontally
        let horizontal_layout_2: [Rect; 2] =
            Layout::horizontal([Constraint::Length(50), Constraint::Min(50)])
                .areas(*vertical_layout.get(2).unwrap());
        // collections window
        let collections_widget = List::new(self.collections.clone())
            .block(
                theme::set_border_style(
                    self.focused_window == FocusedWindow::Collections,
                    self.theme.clone(),
                )
                .unwrap()
                .title(format!("[1] Collections  {}", self.selected_collection)),
            )
            .style(
                theme::match_color_theme_for_widgets(
                    self.theme.clone(),
                    ThemeState::Normal,
                    WidgetType::List,
                )
                .unwrap(),
            )
            .highlight_style(
                theme::match_color_theme_for_widgets(
                    self.theme.clone(),
                    ThemeState::Focus,
                    WidgetType::List,
                )
                .unwrap(),
            );
        frame.render_stateful_widget(
            collections_widget,
            *horizontal_layout_2.get(0).unwrap(),
            &mut self.collection_window_list_state,
        );

        let sub_vertical_layout_right: [Rect; 2] =
            Layout::vertical([Constraint::Length(50), Constraint::Min(30)])
                .areas(*horizontal_layout_2.get(1).unwrap());

        //
        //
        // Request Widget
        //
        //
        let requests_widget = Tabs::new(RequestWidgetTabs::iter().map(|tab| tab.to_string()))
            .select(self.selected_tab)
            .block(
                theme::set_border_style(
                    self.focused_window == FocusedWindow::Request,
                    self.theme.clone(),
                )
                .unwrap()
                .title("[2] Requests"),
            )
            .divider("")
            .style(
                theme::match_color_theme_for_widgets(
                    self.theme.clone(),
                    ThemeState::Normal,
                    WidgetType::Tab,
                )
                .unwrap(),
            )
            .highlight_style(
                theme::match_color_theme_for_widgets(
                    self.theme.clone(),
                    ThemeState::Focus,
                    WidgetType::Tab,
                )
                .unwrap(),
            );

        let request_widget_parent_container = sub_vertical_layout_right.get(0).unwrap();

        frame.render_widget(requests_widget, *request_widget_parent_container);

        // select the right content to display using the select tab
        let current_request_widget_content = manager::match_request_widget_with_opened_tab(
            RequestWidgetTabs::iter().nth(self.selected_tab).unwrap(),
        )
        .unwrap();

        // adjust the child Rec based on the parent to load request content
        let request_widget_child_container = Rect::new(
            request_widget_parent_container.x + 1,
            request_widget_parent_container.y + 2,
            request_widget_parent_container.width - 2,
            request_widget_parent_container.height,
        );

        frame.render_widget(
            current_request_widget_content,
            request_widget_child_container,
        );

        //
        //
        // Response Widget
        //
        //
        let response_widget = Tabs::new(ResponseWidgetTabs::iter().map(|tab| tab.to_string()))
            .select(self.selected_response_tab)
            .block(
                theme::set_border_style(
                    self.focused_window == FocusedWindow::Response,
                    self.theme.clone(),
                )
                .unwrap()
                .title("[3] Response"),
            )
            .divider("")
            .style(
                theme::match_color_theme_for_widgets(
                    self.theme.clone(),
                    ThemeState::Normal,
                    WidgetType::Tab,
                )
                .unwrap(),
            )
            .highlight_style(
                theme::match_color_theme_for_widgets(
                    self.theme.clone(),
                    ThemeState::Focus,
                    WidgetType::Tab,
                )
                .unwrap(),
            );

        let response_widget_parent_container = sub_vertical_layout_right.get(1).unwrap();

        frame.render_widget(response_widget, *response_widget_parent_container);

        // select the right content to display using the select tab
        let current_response_widget_content = manager::match_response_widget_with_opened_tab(
            ResponseWidgetTabs::iter()
                .nth(self.selected_response_tab)
                .unwrap(),
        )
        .unwrap();

        // adjust the child Rec based on the parent to load request content
        let response_widget_child_container = Rect::new(
            response_widget_parent_container.x + 1,
            response_widget_parent_container.y + 2,
            response_widget_parent_container.width - 2,
            response_widget_parent_container.height,
        );

        frame.render_widget(
            current_response_widget_content,
            response_widget_child_container,
        );
    }
}
