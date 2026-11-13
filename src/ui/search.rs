use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    symbols::border,
    widgets::{Block, Paragraph, Widget},
};

pub fn render_search(input: &str, area: Rect, buf: &mut Buffer) {
    let block = Block::bordered()
        .border_set(border::THICK)
        .title(" Rustcher ");

    let paragraph = if input.is_empty() {
        Paragraph::new(" Type to Search ").style(Style::default().fg(Color::White))
    } else {
        Paragraph::new(format!("{input} "))
    };

    paragraph.block(block).render(area, buf);
}
