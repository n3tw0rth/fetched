use ratatui::layout::Rect;
use ratatui::widgets::ListState;
use serde::{Deserialize, Serialize};

use crate::core::enums::{FocusedWindow, InputMode, InputStrategy, LogTypes, WindowOperation};
use crate::core::theme;
use std::collections::HashMap;

//App holds the state of the application
pub struct App {
    pub request_data: RequestStructure,
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

//pub struct RequestWidget {
//    pub tabs: RequestWidgetTabs,
//}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct RequestStructure {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub query_parameters: HashMap<String, String>,
    pub body_type: String,
    pub body: String,
    pub options: RequestOptions,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct RequestOptions {
    validate_ssl: bool,
    follow_redirect: bool,
    attach_cookies: bool,
    proxy: String,
    timeout: u8,
}
