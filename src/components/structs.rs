use ratatui::layout::Rect;
use ratatui::widgets::{ListItem, ListState};

use crate::core::enums::{
    FocusedWindow, InputMode, InputStrategy, RequestWidgetTabs, WindowOperation,
};
use crate::core::theme;
use std::collections::HashMap;

//App holds the state of the application
pub struct App {
    pub rectangles: HashMap<String, Rect>,
    pub input_buffer: HashMap<u8, String>,
    pub theme: theme::Config,
    // Current value of the input box
    pub input: String,
    // Position of cursor in the editor area.
    pub character_index: usize,
    // Current input mode
    pub input_mode: InputMode,
    // Input strategy
    pub input_strategy: InputStrategy,
    // Focused window
    pub focused_window: FocusedWindow,
    // state
    // collections
    pub collections: Vec<String>,
    pub collection_window_list_state: ListState,
    pub selected_collection: String,
    pub show_collection_children: bool,
    // request
    pub selected_tab: usize,
    // response
    pub selected_response_tab: usize,
    // operation
    pub current_operation: WindowOperation,
    // common attr to decide which element to focus on
    pub sub_focus_element: u8,
}

pub struct RequestWidget {
    pub tabs: RequestWidgetTabs,
}
