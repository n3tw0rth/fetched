use ratatui::layout::Rect;
use ratatui::widgets::{ListItem, ListState};
use serde_json::Value;

use crate::core::enums::{
    FocusedWindow, InputMode, InputStrategy, LogTypes, RequestWidgetTabs, WindowOperation,
};
use crate::core::theme;
use std::collections::HashMap;

//App holds the state of the application
pub struct App {
    pub request_data: Value,
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
    pub selected_request: String,
    pub show_collection_children: bool,
    // request
    pub selected_tab: usize,
    // response
    pub selected_response_tab: usize,
    // operation
    pub current_operation: WindowOperation,
    // common attr to decide which element to focus on
    pub sub_focus_element: u8,
    // pop up
    pub is_show_popup: bool,
    pub popup_msg: String,
    pub popup_type: LogTypes,
}

pub struct RequestWidget {
    pub tabs: RequestWidgetTabs,
}
