use tui::{
    style::{Color, Modifier, Style},
    text::{Span, Spans},
};

use crate::state;

pub fn parse(link_scroll: u16, scroll: u16, content: &str) -> Vec<Spans> {
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
        .bg(Color::Black)
        .add_modifier(Modifier::ITALIC);
    let mut link_counter: u16 = 0;
    let mut url_vec: Vec<String> = Vec::new();

    content
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
                                x = t[1..].join(" ");
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
        .collect()
}

pub fn extract_link(content: &str, link_scroll: u16, state: &state::ApplicationState) -> String {
    let mut link_counter: u16 = 0;
    let mut return_val: String = "invalid_entry_12345678".to_string();
    content.split('\n').for_each(|x| {
        if x.len() > 1 {
            match x.chars().nth(0).unwrap() {
                '=' => {
                    if x.chars().nth(1).unwrap() == '>' {
                        let mut x = x.to_string();
                        x.remove(0);
                        x.remove(0);
                        x = x.trim().to_string();
                        let t = x.split_whitespace().nth(0).unwrap();
                        if link_counter == link_scroll {
                            if x.split_whitespace().collect::<Vec<&str>>().len() > 1 {
                                if !t.to_string().starts_with("gemini://") {
                                    return_val = state.history[state.history.len() - 1].clone();
                                    if return_val.ends_with('/') {
                                        return_val += t;
                                    } else {
                                        return_val += "/";
                                        return_val += t;
                                    }
                                } else {
                                    return_val = t.to_string();
                                }
                            } else {
                                return_val = x;
                            }
                            link_counter += 1;
                        } else {
                            link_counter += 1;
                        }
                    }
                }
                _ => {}
            }
        }
    });
    return return_val;
}
