use ratatui::widgets::ListState;

use crate::core::enums::{FocusedWindow, InputMode, InputStrategy, RequestWidgetTabs};
use crate::core::theme;

//App holds the state of the application
pub struct App {
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
    pub collection_window_list_state: ListState,
    // request
    pub selected_tab: usize,
}

pub struct RequestWidget {
    pub tabs: RequestWidgetTabs,
}
