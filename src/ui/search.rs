use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

pub fn render_search(area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .border_set(border::THICK)
        .title(" Sertch Ap ");

    Paragraph::new("Type to search ")
        .centered()
        .block(block)
        .render(area, buf)
}
