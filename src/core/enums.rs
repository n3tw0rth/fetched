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
