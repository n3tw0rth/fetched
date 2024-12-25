use ratatui::style::{Color, Modifier, Style};
use ratatui::widgets::Block;
use serde::Deserialize;
use std::fs;
use toml::de::Error;

use crate::core::enums::{ThemeAttribute, ThemeState, WidgetType};

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub focus: Colors,
    pub normal: Colors,
}

#[derive(Debug, Deserialize, Clone)]
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
            .title_alignment(ratatui::layout::Alignment::Left)
            .style(Style::new().bg(Color::from_u32(theme.focus.background))))
    } else {
        Ok(Block::bordered()
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

// WIP
//
// set the border when input elements are focused
pub fn set_border_color(is_focused: boolean) ->  {}
