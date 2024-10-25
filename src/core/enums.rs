#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Editing,
}

#[derive(PartialEq)]
pub enum InputStrategy {
    Search,  // accessed using /
    Command, // accessed using :
}
