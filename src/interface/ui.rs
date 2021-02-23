use tui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Style},
    text::Span,
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub fn ret(scroll: u16, content: String) -> (Paragraph<'static>, Paragraph<'static>) {
    (Paragraph::new(">")
        .style(Style::default())
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((scroll, 0)), 
    Paragraph::new(content)
        .style(Style::default())
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default())
                .title(Span::styled("Gremlin", Style::default())),
        )
        .alignment(Alignment::Left)
        .wrap(Wrap { trim: true })
        .scroll((scroll, 0)))
}
