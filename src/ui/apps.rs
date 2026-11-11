use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    widgets::{Block, Widget},
};

use crate::{app::App, ui::list};

pub fn render_apps(app: &App, area: Rect, buf: &mut Buffer) {
    let apps_block = Block::bordered()
        .title_top(" Apps ")
        .border_set(border::THICK);

    let inner_area = apps_block.inner(area);

    apps_block.render(area, buf);

    list::render_apps_list(
        &app.list.items,
        app.list.list_state.selected(),
        inner_area,
        buf,
    );
}
