pub mod apps;
pub mod list;
pub mod search;

use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
};

use crate::app::App;

pub fn render(app: &App, area: Rect, buf: &mut Buffer) {
    let [search_area, apps_area] =
        Layout::vertical([Constraint::Length(3), Constraint::Min(0)]).areas(area);

    search::render_search(&app.input, search_area, buf);
    apps::render_apps(app, apps_area, buf);
}
