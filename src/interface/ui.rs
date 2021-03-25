use tui::{
    layout::{Alignment, Constraint, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, BorderType, Borders, Paragraph, Wrap},
};

pub fn ret(link_scroll: u16, scroll: u16, content: &str) -> (bool, Vec<Paragraph>) {
    let style_heading: Style = Style::default().fg(Color::Red).add_modifier(Modifier::BOLD);
    let style_sub_heading: Style = Style::default()
        .fg(Color::Magenta)
        .add_modifier(Modifier::BOLD)
        .add_modifier(Modifier::ITALIC);
    let style_link: Style = Style::default()
        .fg(Color::Blue)
        .add_modifier(Modifier::ITALIC);
    let style_selected_link: Style = Style::default()
        .fg(Color::Blue)
        .bg(Color::Yellow)
        .add_modifier(Modifier::ITALIC);
    let mut link_counter: u16 = 0;
    let mut url_vec: Vec<String> = Vec::new();

    let content: Vec<Spans> = content
        .split('\n')
        .map(|x| {
            if x.len() > 0 {
                match x.chars().nth(0).unwrap() {
                    '#' => {
                        if x.len() > 1 && x.chars().nth(1).unwrap() == '#' {
                            let mut x = x.to_string();
                            x.remove(0);
                            x.remove(0);
                            x = x.trim().to_string();
                            Spans::from(Span::styled(x, style_sub_heading))
                        } else {
                            let mut x = x.to_string();
                            x.remove(0);
                            x = x.trim().to_string();
                            Spans::from(Span::styled(x, style_heading))
                        }
                    }
                    '=' => {
                        if x.len() > 1 && x.chars().nth(1).unwrap() == '>' {
                            link_counter += 1;
                            let mut x = x.to_string();
                            x.remove(0);
                            x.remove(0);
                            x = x.trim().to_string();
                            let t = x.split_whitespace().collect::<Vec<&str>>();
                            if x.split_whitespace().collect::<Vec<&str>>().len() > 1 {
                                url_vec.push(t[0].to_string().clone());
                                x = t[1..].join("");
                            } else {
                                url_vec.push(x.clone());
                            }
                            if link_counter == link_scroll + 1 {
                                Spans::from(Span::styled(x, style_selected_link))
                            } else {
                                Spans::from(Span::styled(x, style_link))
                            }
                        } else {
                            Spans::from(x)
                        }
                    }
                    _ => Spans::from(x),
                }
            } else {
                Spans::from(x)
            }
        })
        .collect();
    (
        link_scroll >= link_counter,
        vec![Paragraph::new(content)
            .style(Style::default())
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default())
                    .title(Span::styled("Gremlin", Style::default())),
            )
            .alignment(Alignment::Left)
            .wrap(Wrap { trim: true })
            .scroll((scroll, 0))],
    )
}
