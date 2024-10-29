use strum::{Display, EnumIter, FromRepr};

#[derive(Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(Clone, PartialEq)]
pub enum InputStrategy {
    Search,  // accessed using /
    Command, // accessed using :
}

#[derive(PartialEq)]
pub enum FocusedWindow {
    Collections,
    Request,
    Input,
}

pub enum WindowMotion {
    Up,
    Down,
    Left,
    Right,
    Expand,
    Collapse,
}

// widgets
#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Tab 1")]
    Tab1,
    #[strum(to_string = "Tab 2")]
    Tab2,
    #[strum(to_string = "Tab 3")]
    Tab3,
    #[strum(to_string = "Tab 4")]
    Tab4,
}
