use ratatui::style::{Color, Style};
use ratatui::widgets::{Block, BorderType};
use serde::Deserialize;
use std::fs;
use toml::de::Error;

use crate::core::enums::{ThemeState, WidgetType};

#[derive(Default, Debug, Clone, Deserialize)]
pub struct Config {
    pub focus: Colors,
    pub normal: Colors,
}

#[derive(Default, Debug, Deserialize, Clone)]
pub struct Colors {
    pub foreground: u32,
    pub background: u32,
    pub highlight: u32,
    pub border: u32,
}

pub fn get_theme() -> Result<Config, Error> {
    let config: Config = toml::from_str(&fs::read_to_string("theme.toml").expect(""))
        .expect("Failed to parse theme");

    Ok(config)
}

pub fn set_border_style<'a>(
    is_window_selected: bool,
    theme: Config,
) -> Result<Block<'a>, Box<dyn std::error::Error>> {
    if is_window_selected {
        Ok(Block::bordered()
            .border_type(BorderType::Rounded)
            .title_alignment(ratatui::layout::Alignment::Left)
            .style(Style::new().fg(Color::from_u32(theme.focus.highlight))))
    } else {
        Ok(Block::bordered()
            .border_type(BorderType::Rounded)
            .title_alignment(ratatui::layout::Alignment::Left)
            .style(
                Style::new(), //.bg(Color::from_u32(theme.normal.background))
            ))
    }
}

pub fn match_color_theme_for_widgets(
    theme: Config,
    theme_state: ThemeState,
    widget: WidgetType,
) -> Result<Style, Box<dyn std::error::Error>> {
    match widget {
        WidgetType::Paragraph => match theme_state {
            ThemeState::Focus => Ok(Style::new().fg(Color::from_u32(theme.focus.foreground))),
            ThemeState::Normal => todo!(),
        },
        WidgetType::Tab => match theme_state {
            ThemeState::Focus => Ok(Style::new().fg(Color::from_u32(theme.focus.foreground))),
            ThemeState::Normal => Ok(Style::new().fg(Color::from_u32(theme.normal.foreground))),
        },
        WidgetType::List => match theme_state {
            ThemeState::Focus => Ok(Style::new()
                .fg(Color::from_u32(theme.focus.foreground))
                .bg(Color::from_u32(theme.focus.highlight))),
            ThemeState::Normal => Ok(Style::new().fg(Color::from_u32(theme.normal.foreground))),
        },
    }
}

// set the border when input elements are focused
pub fn set_input_block<'a>(is_focused: bool) -> Block<'a> {
    let theme = get_theme().unwrap();
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(if is_focused {
            Color::from_u32(theme.focus.border)
        } else {
            Color::from_u32(theme.normal.border)
        });
    block
}

pub fn set_button_block<'a>(is_focused: bool) -> Block<'a> {
    let theme = get_theme().unwrap();
    let block = Block::bordered()
        .border_type(BorderType::Rounded)
        .border_style(if is_focused {
            Color::from_u32(theme.focus.border)
        } else {
            Color::from_u32(theme.normal.border)
        });
    block
}

pub fn set_button_style(is_focused: bool) -> Style {
    let theme = get_theme().unwrap();
    let style = Style::new().fg(if is_focused {
        Color::from_u32(theme.focus.foreground)
    } else {
        Color::from_u32(theme.normal.foreground)
    });
    style
}
