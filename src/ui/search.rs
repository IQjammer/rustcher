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
        .title(" Sertch Ap ");

    let paragraph = if input.is_empty() {
        Paragraph::new(" Type to Sertch ").style(Style::default().fg(Color::DarkGray))
    } else {
        Paragraph::new(format!("{input} "))
    };

    paragraph.block(block).render(area, buf);
}
