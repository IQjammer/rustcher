use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph, Widget},
};
use ratatui_image::Image;

pub fn render_apps_list(
    items: &[&crate::desktop::AppEntry],
    selected_index: Option<usize>,
    area: Rect,
    buf: &mut Buffer,
) {
    if items.is_empty() || area.height == 0 {
        return;
    }

    const ITEM_HEIGHT: u16 = 4;

    let visible_count = (area.height / ITEM_HEIGHT).max(1) as usize;
    let selected = selected_index.unwrap_or(0);

    let half_visible = visible_count / 2;
    let max_offset = items.len().saturating_sub(visible_count);
    let offset = selected.saturating_sub(half_visible).min(max_offset);

    let end = (offset + visible_count).min(items.len());
    let visible_items = &items[offset..end];

    let constraints = vec![Constraint::Length(ITEM_HEIGHT); visible_items.len()];
    let chunks = Layout::vertical(constraints).split(area);

    for (rel_i, (item, &chunk)) in visible_items.iter().zip(chunks.iter()).enumerate() {
        let abs_i = offset + rel_i;
        let is_selected = selected_index == Some(abs_i);

        let border_color = if is_selected {
            Color::Green
        } else {
            Color::White
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::THICK)
            .border_style(Style::default().fg(border_color));

        let inner = block.inner(chunk);
        block.render(chunk, buf);

        let content_chunks =
            Layout::horizontal([Constraint::Length(5), Constraint::Min(0)]).split(inner);

        if let Some(ref protocol_img) = item.icon {
            let image_widget = Image::new(protocol_img);
            image_widget.render(content_chunks[0], buf);
        } else {
            let icon_widget = Paragraph::new("no").style(Style::default().fg(if is_selected {
                Color::Green
            } else {
                Color::White
            }));
            icon_widget.render(content_chunks[0], buf);
        }

        let text = vec![
            Line::from(vec![Span::styled(
                &item.name,
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![Span::styled(
                &item.description,
                Style::default().fg(Color::DarkGray),
            )]),
        ];

        let text_widget = Paragraph::new(text);
        text_widget.render(content_chunks[1], buf)
    }
}
