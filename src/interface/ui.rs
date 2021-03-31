use tui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub fn build(content: Vec<Spans>, scroll: u16) -> Paragraph {
    Paragraph::new(content.clone())
                        .style(Style::default())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .style(Style::default())
                                .title(Span::styled("Gremlin", Style::default())),
                        )
                        .alignment(Alignment::Left)
                        .wrap(Wrap { trim: true })
                        .scroll((scroll as u16, 0))
}
