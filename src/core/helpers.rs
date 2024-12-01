use ratatui::layout::Rect;

use crate::core::enums::ContainerPositions;

pub fn logger(msg: String) {
    use std::fs::OpenOptions;
    use std::io::{self, Write};

    // Open the file in write mode (create if not exists).
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true)
        .open("runtime.log")
        .unwrap();

    writeln!(file, "{}", msg).unwrap();
}

pub fn clear_logger() {
    use std::fs::OpenOptions;
    use std::io::{self, Write};

    // Open the file in write mode (create if not exists).
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open("runtime.log")
        .unwrap();

    writeln!(file, "{}", "").unwrap();
}

// get the desired rectangle
// TODO: add orientation
pub fn find_position(position: ContainerPositions, ratio: f32, area: Rect) -> Rect {
    let height = (area.height as f32 * ratio) as u16;
    match position {
        ContainerPositions::Bottom => {
            Rect::new(area.left(), area.bottom() - height, area.width, height)
        }
        _ => area,
    }
}
