use tui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub fn build_main(content: Vec<Spans>, scroll: u16) -> Paragraph {
    Paragraph::new(content)
                        .style(Style::default())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .style(Style::default())
                                .title(Span::styled("Gremlin", Style::default())),
                        )
                        .alignment(Alignment::Left)
                        .wrap(Wrap {trim: false})
                        .scroll((scroll as u16, 0))
}

pub fn build_input(content: String) -> Paragraph<'static> {
    Paragraph::new(content)
                        .style(Style::default())
                        .block(
                            Block::default()
                                .borders(Borders::ALL)
                                .style(Style::default())
                                .title(Span::styled("Enter your URI/URL here", Style::default())),
                        )
                        .alignment(Alignment::Left)
                        .wrap(Wrap {trim: false})
}
