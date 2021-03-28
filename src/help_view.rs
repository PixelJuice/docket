use tui::widgets::{Block, Borders, Paragraph};
use tui::style::{Color, Modifier, Style};
use tui::text::{Span, Spans};
use crossterm::{
    Result,
};

pub fn build_view(active: &bool) -> Paragraph {

    let mut color = Color::White;
    let mut highlight_color = Color::Green;

    if !active {
        color = Color::DarkGray;
        highlight_color = Color::DarkGray;
    }

    let menu_titles = vec![". Toggle Help", "New Item"];
    let mut menu: Vec<tui::text::Spans> = menu_titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(highlight_color)
                        .add_modifier(Modifier::UNDERLINED),
                ),
                Span::styled(rest, Style::default().fg(color)),
            ])
        })
        .collect();
    menu.push(Spans::from(vec![
        Span::styled(
            "[Enter]",
            Style::default()
                .fg(highlight_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Edit Item", Style::default().fg(color)),
    ]));
    menu.push(Spans::from(vec![
        Span::styled(
            "[Space]",
            Style::default()
                .fg(highlight_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled(" Check/Uncheck Item", Style::default().fg(color)),
    ]));
    menu.push(Spans::from(vec![
        Span::styled(
            "Q",
            Style::default()
                .fg(highlight_color)
                .add_modifier(Modifier::BOLD),
        ),
        Span::styled("uit", Style::default().fg(color)),
    ]));

    Paragraph::new(menu.clone())
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(color))
}