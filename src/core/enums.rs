use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter, FromRepr};

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

pub enum WidgetType {
    Paragraph,
    Tab,
    List,
}

pub enum ThemeState {
    Focus,
    Normal,
}

pub enum ThemeAttribute {
    Foreground,
    Background,
    Highlight,
    Border,
}

// widgets
#[derive(Default, Debug, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum RequestWidgetTabs {
    #[default]
    #[strum(to_string = "Header")]
    Header,
    #[strum(to_string = "Body")]
    Body,
    #[strum(to_string = "Query")]
    Query,
    #[strum(to_string = "Authentication")]
    Authentication,
}
