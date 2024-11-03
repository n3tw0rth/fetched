use std::error::Error;

use ratatui::widgets::Paragraph;

use super::widget_contents;
use crate::core::enums::{RequestWidgetTabs, ResponseWidgetTabs};

pub fn match_request_widget_with_opened_tab<'a>(
    selected_tab: RequestWidgetTabs,
) -> Result<Paragraph<'a>, Box<dyn Error>> {
    match selected_tab {
        RequestWidgetTabs::Body => widget_contents::body::render(),
        RequestWidgetTabs::Query => widget_contents::query::render(),
        RequestWidgetTabs::Header => widget_contents::header::render(),
        RequestWidgetTabs::Authentication => widget_contents::auth::render(),
    }
}

pub fn match_response_widget_with_opened_tab<'a>(
    selected_tab: ResponseWidgetTabs,
) -> Result<Paragraph<'a>, Box<dyn Error>> {
    match selected_tab {
        ResponseWidgetTabs::ResponseBody => widget_contents::response_body::render(),
        ResponseWidgetTabs::ResponseQuery => widget_contents::response_body::render(),
    }
}
