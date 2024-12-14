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

pub fn get_inner(rect: Rect, left: u16, top: u16, right: u16, bottom: u16) -> Rect {
    let padded_rect = Rect::new(
        rect.x + left,
        rect.y + top,
        rect.width - right,
        rect.height.saturating_sub(bottom),
    );
    padded_rect
}

pub fn get_width_by_ratio(width: u16, slices: Vec<u16>) -> Vec<u16> {
    slices
        .iter()
        .map(|i| width.saturating_div(10) * i)
        .collect()
}

pub fn find_position(position: ContainerPositions, ratio: f32, area: Rect) -> Rect {
    let height = (area.height as f32 * ratio) as u16;
    match position {
        ContainerPositions::Bottom => {
            Rect::new(area.left(), area.bottom() - height, area.width, height)
        }
        _ => area,
    }
}
