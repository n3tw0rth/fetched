use std::collections::HashMap;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::widgets::Tabs;
use ratatui::Frame;
use strum::IntoEnumIterator;

use crate::components::{drawable, manager};
use crate::core::enums::{
    FocusedWindow, InputMode, RequestWidgetTabs, ThemeState, WidgetType, WindowOperation,
};
use crate::core::{helpers, theme};

pub fn draw_response_widget(
    current_theme: &theme::Config,
    selected_tab: usize,
    frame: &mut Frame,
    input_buffer: &mut HashMap<u8, String>,
    focused_window: &FocusedWindow,
    input_mode: &InputMode,
    current_operation: WindowOperation,
    sub_focus_element: u8,
    area: Rect,
) {
    let requests_widget = Tabs::new(RequestWidgetTabs::iter().map(|tab| tab.to_string()))
        .select(selected_tab)
        .block(
            theme::set_border_style(
                *focused_window == FocusedWindow::Request,
                current_theme.clone(),
            )
            .unwrap()
            .title("[2] Requests"),
        )
        .divider("")
        .style(
            theme::match_color_theme_for_widgets(
                current_theme.clone(),
                ThemeState::Normal,
                WidgetType::Tab,
            )
            .unwrap(),
        )
        .highlight_style(
            theme::match_color_theme_for_widgets(
                current_theme.clone(),
                ThemeState::Focus,
                WidgetType::Tab,
            )
            .unwrap(),
        );

    frame.render_widget(requests_widget, area);

    // select the right content to display using the select tab
    let current_request_widget_content = manager::match_request_widget_with_opened_tab(
        RequestWidgetTabs::iter().nth(selected_tab).unwrap(),
    )
    .unwrap();

    // adjust the child Rec based on the parent to load request content
    let request_widget_child_containers: [Rect; 2] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(4)]).areas(area);
    let request_widget_child_container_content =
        helpers::get_inner(*request_widget_child_containers.get(0).unwrap(), 1, 2, 2, 1);
    let request_widget_child_container_input =
        helpers::get_inner(*request_widget_child_containers.get(1).unwrap(), 1, 0, 2, 1);

    frame.render_widget(
        current_request_widget_content,
        request_widget_child_container_content,
    );

    // render request component sub components bound to operations
    match focused_window {
        FocusedWindow::Request => match current_operation {
            WindowOperation::Edit => {
                if *input_mode == InputMode::Insert {
                    drawable::editheader::draw(
                        frame,
                        request_widget_child_container_input,
                        sub_focus_element,
                        input_buffer,
                    );
                }
            }
            _ => {}
        },
        _ => {}
    }
}
