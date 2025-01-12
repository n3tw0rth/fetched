use strum_macros::{Display, EnumIter, FromRepr};

#[derive(Clone, PartialEq)]
pub enum InputMode {
    Normal,
    Insert,
    Control,
}

#[derive(Clone, PartialEq)]
pub enum InputStrategy {
    Search,  // accessed using /
    Command, // accessed using :
    Prompt,
}

#[derive(Clone, PartialEq)]
pub enum FocusedWindow {
    Collections,
    Request,
    Response,
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

#[derive(Clone, Copy)]
pub enum WindowOperation {
    Create,
    Delete,
    Rename,
    Edit,
    Open,
    Null,
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

pub enum ContainerPositions {
    Top,
    Bottom,
    Middle,
    Center,
    TopCenter,
    BottomCenter,
    MiddleCenter,
    CenterCenter,
    Full,
}

pub enum LayoutOrientation {
    Vertical,
    Horizontal,
}

#[derive(Default, Debug, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum LogTypes {
    #[default]
    #[strum(to_string = "Info")]
    Info,
    #[strum(to_string = "Error")]
    Error,
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

#[derive(Debug, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum ResponseWidgetTabs {
    //#[strum(to_string = "Header")]
    //Header,
    #[strum(to_string = "Body")]
    ResponseBody,
    #[strum(to_string = "Header")]
    ResponseHeader,
    //#[strum(to_string = "Authentication")]
    //Authentication,
}
