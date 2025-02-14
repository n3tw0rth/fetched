use crate::components::structs::{App, RequestStructure};
use crate::components::{drawable, events, manager, widgets};
use crate::core::enums::{
    FocusedWindow, InputMode, InputStrategy, LogTypes, RequestWidgetTabs, ResponseWidgetTabs,
    ThemeState, WidgetType, WindowMotion, WindowOperation,
};
use crate::core::request_parser;
use crate::core::theme;
use crate::core::{handler, helpers};
use color_eyre::Result;
use crossterm::event::KeyModifiers;
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use crossterm::ExecutableCommand;
use ratatui::backend::CrosstermBackend;
use ratatui::layout::Flex;
use ratatui::widgets::{BorderType, Clear, ScrollbarState};
use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Alignment, Constraint, Layout, Position, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, List, ListState, Paragraph, Tabs},
    DefaultTerminal, Frame,
};
use std::collections::HashMap;
use std::error::Error;
use std::io::stdout;
use std::path::PathBuf;
use std::process::Command;
use strum::IntoEnumIterator;

type Terminal = ratatui::Terminal<CrosstermBackend<std::io::Stdout>>;

impl App {
    pub fn new() -> Self {
        Self {
            request_data: RequestStructure::default(),
            rectangles: HashMap::new(),
            input_buffer: HashMap::new(),
            theme: theme::get_theme().unwrap(),
            input: String::new(),
            input_mode: InputMode::Normal,
            input_strategy: InputStrategy::Command,
            character_index: 0,
            focused_window: FocusedWindow::Collections,
            //state
            collections: handler::list_collections(),
            collection_window_list_state: ListState::default().with_selected(Some(0)),
            selected_collection: "".to_string(),
            selected_request: "".to_string(),
            show_collection_children: false,
            // request tabs
            selected_tab: 0,
            vertical_scroll: 0,
            vertical_scroll_state: ScrollbarState::default(),
            //response tabs
            selected_response_tab: 0,
            current_operation: WindowOperation::Null,
            sub_focus_element: 0,
            is_show_popup: false,
            popup_msg: "".to_string(),
            popup_type: LogTypes::Info,
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
        self.input_buffer
            .insert(self.sub_focus_element, self.input.clone());
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
        self.input_buffer
            .insert(self.sub_focus_element, self.input.clone());
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.chars().count())
    }

    fn reset_cursor(&mut self) {
        self.character_index = 0;
    }

    fn prompt(&mut self, operation: WindowOperation) {
        self.input_mode = InputMode::Control;
        self.input_strategy = InputStrategy::Prompt;

        self.current_operation = operation;
    }

    fn get_rectangle(&self, key: String) -> Rect {
        *self.rectangles.get_key_value(&key).unwrap().1
    }

    fn get_selected_value(&self) -> Result<String, Box<dyn std::error::Error>> {
        helpers::logger(self.collections.get(0));
        Ok(self
            .collections
            .get(self.collection_window_list_state.selected().unwrap())
            .unwrap_or(self.collections.get(0).unwrap())
            .to_string())
    }

    fn run_editor(&self, terminal: &mut Terminal, file: String) -> Result<()> {
        stdout().execute(LeaveAlternateScreen)?;
        disable_raw_mode()?;
        Command::new("nvim").arg(file).status()?;
        stdout().execute(EnterAlternateScreen)?;
        enable_raw_mode()?;
        terminal.clear()?;
        Ok(())
    }

    fn execute_operation_on_selected_window(
        &mut self,
        operation: WindowOperation,
        terminal: Option<&mut Terminal>,
    ) {
        self.current_operation = operation;
        match self.focused_window {
            FocusedWindow::Collections => match operation {
                WindowOperation::Open => {
                    if self.show_collection_children {
                        let file_path = handler::get_file_path(
                            self.selected_collection.clone(),
                            self.get_selected_value().unwrap(),
                        )
                        .unwrap();
                        self.run_editor(&mut terminal.unwrap(), file_path).unwrap()
                    }
                }
                _ => {}
            },
            FocusedWindow::Request => match operation {
                WindowOperation::Edit => self.input_mode = InputMode::Insert,
                _ => {}
            },
            _ => {}
        }
    }

    fn execute_operation_on_selected_window_with_promt(
        &mut self,
        operation: WindowOperation,
        promt: String,
    ) {
        match self.focused_window {
            FocusedWindow::Collections => match operation {
                WindowOperation::Create => {
                    if self.show_collection_children {
                        handler::create_request(&self.selected_collection, promt).unwrap()
                    } else {
                        handler::create_collection(promt).unwrap();
                    }
                }
                WindowOperation::Delete => {
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

    fn get_request_file_path(&self) -> anyhow::Result<PathBuf> {
        let path = std::env::current_dir()
            .unwrap()
            .join(&self.selected_collection)
            .join(self.get_selected_value().unwrap());

        helpers::logger(self.collection_window_list_state.selected());
        Ok(path)
    }

    fn refresh_request_data(&mut self) {
        let json_data = request_parser::read_json_file(&self.get_request_file_path().unwrap());

        self.request_data = json_data.unwrap();
        self.current_operation = WindowOperation::Null;
        self.input_buffer.clear();
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
                    self.collection_window_list_state.select(Some(0));
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
                    self.collection_window_list_state.select(Some(0));
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
                WindowMotion::Down => {
                    self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                    self.vertical_scroll_state =
                        self.vertical_scroll_state.position(self.vertical_scroll)
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

        // refresh data once the selections are changed
        if self.show_collection_children {
            self.refresh_request_data();
        }
    }

    fn submit_message(&mut self) {
        if self.input_strategy == InputStrategy::Prompt {
            self.execute_operation_on_selected_window_with_promt(
                self.current_operation,
                self.input.clone(),
            );
        } else {
            handler::event_handler(self.input_strategy.clone(), self.input.clone(), self);
        }
        //match self.input_strategy{
        //    InputStrategy::Search => {}
        //    InputStrategy::Prompt =>{}
        //    InputStrategy::Command =>{}
        //}
        self.input.clear();
        self.reset_cursor();
        self.input_mode = InputMode::Normal;
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.draw(frame))?;

            if let Event::Key(key) = event::read()? {
                // keybindings
                match self.input_mode {
                    InputMode::Normal => match key.code {
                        KeyCode::Char('c') => {
                            if key.modifiers == KeyModifiers::CONTROL {
                                super::handler::exit_app();
                            }
                        }
                        KeyCode::Char('v') => {
                            if key.modifiers == KeyModifiers::CONTROL {
                                helpers::logger(format!("{:?}", self.request_data))
                            }
                        }
                        KeyCode::Char(':') => {
                            self.input_strategy = InputStrategy::Command;
                            self.input_mode = InputMode::Control;
                        }
                        KeyCode::Char('/') => {
                            self.input_strategy = InputStrategy::Search;
                            self.input_mode = InputMode::Control;
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
                        KeyCode::Char('a') => self.prompt(WindowOperation::Create),
                        KeyCode::Char('d') => self.prompt(WindowOperation::Delete),
                        KeyCode::Char('o') => self.execute_operation_on_selected_window(
                            WindowOperation::Open,
                            Some(&mut terminal),
                        ),
                        KeyCode::Char('i') => {
                            self.execute_operation_on_selected_window(WindowOperation::Edit, None)
                        }
                        KeyCode::Esc => {
                            self.is_show_popup = false;
                            self.input_mode = InputMode::Normal;
                            self.reset_input();
                        }
                        _ => {}
                    },
                    InputMode::Control if key.kind == KeyEventKind::Press => match key.code {
                        KeyCode::Enter => self.submit_message(),
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Left => self.move_cursor_left(),
                        KeyCode::Right => self.move_cursor_right(),
                        KeyCode::Esc => {
                            self.is_show_popup = false;
                            self.input_mode = InputMode::Normal;
                            self.reset_input();
                        }
                        _ => {}
                    },
                    InputMode::Insert if key.kind == KeyEventKind::Press => match key.code {
                        // register keypresses in insert mode too
                        KeyCode::Char(to_insert) => self.enter_char(to_insert),
                        KeyCode::Backspace => self.delete_char(),
                        KeyCode::Esc => {
                            self.is_show_popup = false;
                            self.input_mode = InputMode::Normal;
                            self.reset_input();
                        }
                        KeyCode::Tab => self.handle_tab_key(),
                        KeyCode::Enter => self.handle_enter_on_insert_mode(),
                        _ => {}
                    },
                    InputMode::Control => {}
                    InputMode::Insert => {}
                }
            }
        }
    }

    fn popup_area(&self, area: Rect, percent_x: u16, percent_y: u16) -> Rect {
        let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
        let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
        let [area] = vertical.areas(area);
        let [area] = horizontal.areas(area);
        area
    }

    pub fn show_popup(&mut self, msg: String) {
        self.is_show_popup = true;
        self.popup_msg = msg
    }

    // saving the header in the json
    fn handle_enter_on_insert_mode(&mut self) {
        match self.focused_window {
            FocusedWindow::Request => match self.selected_tab {
                0 => {
                    // accept enter only when focused on the add element
                    if self.sub_focus_element == 2 {
                        let _ = events::enter::request_widget_edit_headers_enter_event(
                            &self.input_buffer,
                            &self.get_request_file_path().unwrap(),
                        )
                        .unwrap();
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    // use this method to update the input buufer when the sub_focus_element value changes
    fn update_sub_focus_element(&mut self, value: u8) {
        self.sub_focus_element = value;
        self.input.clear();
    }

    fn handle_tab_key(&mut self) {
        match self.input_mode {
            InputMode::Insert => match self.focused_window {
                FocusedWindow::Request => match self.current_operation {
                    WindowOperation::Edit => {
                        self.update_sub_focus_element(self.sub_focus_element + 1);
                        if self.sub_focus_element > 2 {
                            self.update_sub_focus_element(0);
                        }
                    }
                    _ => {}
                },
                _ => {}
            },
            _ => {}
        }
    }

    fn reset_input(&mut self) {
        self.input.clear();
        self.delete_char();
    }

    fn decide_input_title(&self) -> Result<String, Box<dyn Error>> {
        if self.input_strategy == InputStrategy::Command {
            Ok("Command".to_string())
        } else if self.input_strategy == InputStrategy::Search {
            Ok("Search".to_string())
        } else {
            match self.current_operation {
                WindowOperation::Create => {
                    if self.show_collection_children {
                        Ok("Request Name".to_string())
                    } else {
                        Ok("Collection Name".to_string())
                    }
                }
                WindowOperation::Delete => Ok("Delete Collection [y/N]".to_string()),
                _ => todo!(),
            }
        }
    }

    fn draw(&mut self, frame: &mut Frame) {
        let [header, content, footer] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Min(1),
            Constraint::Length(1),
        ])
        .areas(frame.area());

        let mut vertical_layout: [Rect; 3] = Layout::vertical([
            Constraint::Length(0),
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .areas(content);

        if self.input_mode == InputMode::Control {
            vertical_layout = Layout::vertical([
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .areas(content);
        }

        drawable::header::draw(frame, header);
        drawable::footer::draw(frame, footer, &self.focused_window);

        self.rectangles
            .insert("v0".into(), *vertical_layout.get(0).unwrap());
        self.rectangles
            .insert("v1".into(), *vertical_layout.get(1).unwrap());
        self.rectangles
            .insert("v2".into(), *vertical_layout.get(2).unwrap());

        let input = Paragraph::new(self.input.as_str())
            .style(match self.input_mode {
                InputMode::Normal => Style::default(),
                InputMode::Control => {
                    Style::default().fg(Color::from_u32(self.theme.focus.foreground))
                }
                _ => Style::default(),
            })
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title(self.decide_input_title().unwrap()),
            );
        // render the input field only in editing mode
        if self.input_mode == InputMode::Control {
            frame.render_widget(input, self.get_rectangle("v0".into()))
        }
        match self.input_mode {
            // Hide the cursor. `Frame` does this by default, so we don't need to do anything here
            InputMode::Normal => {}
            // Make the cursor visible and ask ratatui to put it at the specified coordinates after
            // rendering
            #[allow(clippy::cast_possible_truncation)]
            InputMode::Control => frame.set_cursor_position(Position::new(
                // Draw the cursor at the current position in the input field.
                // This position is can be controlled via the left and right arrow key
                self.get_rectangle("v0".into()).x + self.character_index as u16 + 1,
                // Move one line down, from the border to the input line
                self.get_rectangle("v0".into()).y + 1,
            )),
            _ => {}
        }

        // 1st horizontal layout split the 2nd vertical layout horizontally
        let horizontal_layout: [Rect; 2] =
            Layout::horizontal([Constraint::Length(8), Constraint::Min(1)])
                .areas(*vertical_layout.get(1).unwrap());

        self.rectangles
            .insert("h0".into(), *horizontal_layout.get(0).unwrap());
        self.rectangles
            .insert("h1".into(), *horizontal_layout.get(1).unwrap());

        // http method widget
        let http_method_widget = Paragraph::new("POST")
            .style(Style::default().add_modifier(Modifier::BOLD))
            .alignment(Alignment::Center)
            .block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .style(Color::from_u32(self.theme.focus.border)),
            );
        frame.render_widget(http_method_widget, self.get_rectangle("h0".into()));
        // url
        let url_widget = Paragraph::new(self.request_data.url.clone()).block(
            Block::bordered()
                .border_type(BorderType::Rounded)
                .style(Color::White),
        );
        frame.render_widget(url_widget, self.get_rectangle("h1".into()));

        // 2st horizontal layout
        // split the 3nd vertical layout horizontally
        let horizontal_layout_2: [Rect; 2] =
            Layout::horizontal([Constraint::Length(50), Constraint::Min(50)])
                .areas(self.get_rectangle("v2".into()));

        // sb -> sub-horizontal
        self.rectangles
            .insert("sh0".into(), *horizontal_layout_2.get(0).unwrap());
        self.rectangles
            .insert("sh1".into(), *horizontal_layout_2.get(1).unwrap());

        // collections window
        let collections_widget = List::new(
            self.collections
                .clone()
                .iter()
                .map(|item|
                   if self.show_collection_children{
                   "\u{f323} ".to_string() 
                   }else{
                   "\u{f024b} ".to_string() 
                   }
                    + item),
        )
        .block(
            theme::set_border_style(
                self.focused_window == FocusedWindow::Collections,
                self.theme.clone()
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
            self.get_rectangle("sh0".into()),
            &mut self.collection_window_list_state,
        );

        let sub_vertical_layout_right: [Rect; 2] =
            Layout::vertical([Constraint::Length(50), Constraint::Min(30)])
                .areas(self.get_rectangle("sh1".into()));

        self.rectangles
            .insert("sv0".into(), *sub_vertical_layout_right.get(0).unwrap());
        self.rectangles
            .insert("sv1".into(), *sub_vertical_layout_right.get(1).unwrap());

        //
        //
        // Request Widget
        //
        //
        let request_widget_parent_container = self.get_rectangle("sv0".into());

        widgets::request::draw_request_widget(frame, self, request_widget_parent_container);

        //
        //
        // Response Widget
        //
        //
        let response_widget_parent_container = self.get_rectangle("sv1".into());

        widgets::response::draw_response_widget(
            &self.theme,
            self.selected_response_tab,
            frame,
            &self.focused_window,
            response_widget_parent_container,
        );

        if self.is_show_popup {
            //let cowsay = Command::new("cowsay")
            //    .arg(self.popup_msg.clone())
            //    .output()
            //    .unwrap()
            //    .stdout;
            let msg = Paragraph::new(self.popup_msg.clone()).block(
                Block::bordered()
                    .border_type(BorderType::Rounded)
                    .title_top(self.popup_type.to_string()),
            );
            let area = self.popup_area(frame.area(), 60, 20);
            frame.render_widget(Clear, area); //this clears out the background
            frame.render_widget(msg, area);
        }
    }
}
