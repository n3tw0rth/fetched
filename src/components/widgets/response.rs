use ratatui::layout::{Margin, Rect};
use ratatui::widgets::Tabs;
use ratatui::Frame;
use strum::IntoEnumIterator;

use crate::components::manager;
use crate::core::enums::{FocusedWindow, ResponseWidgetTabs, ThemeState, WidgetType};
use crate::core::theme;

pub fn draw_response_widget(
    current_theme: &theme::Config,
    selected_tab: usize,
    frame: &mut Frame,
    focused_window: &FocusedWindow,
    area: Rect,
) {
    let response_widget = Tabs::new(ResponseWidgetTabs::iter().map(|tab| tab.to_string()))
        .select(selected_tab)
        .block(
            theme::set_border_style(
                *focused_window == FocusedWindow::Response,
                current_theme.clone(),
            )
            .unwrap()
            .title("[3] Response"),
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

    frame.render_widget(response_widget, area);

    // select the right content to display using the select tab
    let current_response_widget_content = manager::match_response_widget_with_opened_tab(
        ResponseWidgetTabs::iter().nth(selected_tab).unwrap(),
    )
    .unwrap();

    // adjust the child Rec based on the parent to load request content
    let response_widget_child_container = area.inner(Margin::new(1, 2));

    frame.render_widget(
        current_response_widget_content,
        response_widget_child_container,
    );
}
