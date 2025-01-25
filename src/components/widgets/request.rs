use std::collections::HashMap;

use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::text::Line;
use ratatui::widgets::{Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Tabs};
use ratatui::Frame;
use strum::IntoEnumIterator;

use crate::components::structs::{App, RequestStructure};
use crate::components::{drawable, manager};
use crate::core::enums::{
    FocusedWindow, InputMode, RequestWidgetTabs, ThemeState, WidgetType, WindowOperation,
};
use crate::core::{helpers, theme};

pub fn draw_request_widget(frame: &mut Frame, state: &mut App, area: Rect) {
    let mut scroll_items: Vec<Line> = vec![];
    let requests_widget = Tabs::new(RequestWidgetTabs::iter().map(|tab| tab.to_string()))
        .select(state.selected_tab)
        .block(
            theme::set_border_style(
                state.focused_window == FocusedWindow::Request,
                state.theme.clone(),
            )
            .unwrap()
            .title("[2] Requests"),
        )
        .divider("")
        .style(
            theme::match_color_theme_for_widgets(
                state.theme.clone(),
                ThemeState::Normal,
                WidgetType::Tab,
            )
            .unwrap(),
        )
        .highlight_style(
            theme::match_color_theme_for_widgets(
                state.theme.clone(),
                ThemeState::Focus,
                WidgetType::Tab,
            )
            .unwrap(),
        );

    frame.render_widget(requests_widget, area);

    // select the right content to display using the select tab
    //let current_request_widget_content = manager::match_request_widget_with_opened_tab(
    //    RequestWidgetTabs::iter().nth(state.selected_tab).unwrap(),
    //)
    //.unwrap();

    // adjust the child Rec based on the parent to load request content
    let request_widget_child_containers: [Rect; 2] =
        Layout::vertical([Constraint::Min(1), Constraint::Length(4)]).areas(area);
    let request_widget_child_container_content =
        helpers::get_inner(*request_widget_child_containers.get(0).unwrap(), 1, 2, 2, 1);
    let request_widget_child_container_input =
        helpers::get_inner(*request_widget_child_containers.get(1).unwrap(), 1, 0, 2, 1);

    //frame.render_widget(
    //    current_request_widget_content,
    //    request_widget_child_container_content,
    //);

    manager::match_request_widget_with_opened_tab(
        state,
        frame,
        scroll_items,
        request_widget_child_container_content,
    )
    .unwrap();

    //}

    // render request component sub components bound to operations
    match state.focused_window {
        FocusedWindow::Request => match state.current_operation {
            WindowOperation::Edit => {
                if state.input_mode == InputMode::Insert {
                    drawable::editheader::draw(
                        frame,
                        request_widget_child_container_input,
                        state.sub_focus_element,
                        &mut state.input_buffer,
                    );
                }
            }
            _ => {}
        },
        _ => {}
    }
}
