use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph,BorderType, Clear};
use tui::text::{Span, Spans};
use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    Result,
};

use crate::Context;
use crate::todo_txt::TodoItem;
use crate::todo_txt::TodoTxt;

pub fn run(contex: &mut Context, desc: &mut String, todo_txt: &mut TodoTxt, quit: &mut bool) -> Result<()> {
    match read()? {
        Event::Key(input_event) => {
            if input_event.modifiers == KeyModifiers::CONTROL
                && input_event.code == KeyCode::Char('c')
                || input_event.code == KeyCode::Char('q')
            {
                *quit = true;
            } else if input_event.code == KeyCode::Esc {
                desc.clear();
                *contex = Context::ViewList;
            } else if input_event.code == KeyCode::Backspace {
                desc.pop();
            }
            else if input_event.code == KeyCode::Enter {
                let todo_item = TodoItem::new(desc.to_owned());
                todo_txt.content.push(todo_item);
                todo_txt.sort();
                desc.clear();
                *contex = Context::ViewList;
            }
            else {
                match input_event.code {
                    KeyCode::Char(c) => desc.push(c),
                    _ => {}
                }
            }
        }
        Event::Mouse(_event) => (),
        Event::Resize(_width, _height) => (),
    }
    Ok(())
}

pub fn render(area: &mut tui::Frame<tui::backend::CrosstermBackend<std::io::Stdout>>, desc: &String) {

    let color = Color::White;
    let highlight_color = Color::Cyan;

    let popup_block = Block::default()
    .title("Description")
    .borders(Borders::ALL)
    .border_style(Style::default().fg(Color::White))
    .border_type(BorderType::Double)
    .style(Style::default().bg(Color::Black));

    let message: Vec<tui::text::Spans> = vec![Spans::from(vec![
        Span::styled(desc, Style::default().fg(highlight_color))])];

    let input = Paragraph::new(message)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(color));

    let popupsize= Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Length(5),
                ]
                .as_ref(),
            )
            .split(area.size())[0];
    let input_field = Layout::default()
    .direction(Direction::Vertical)
    .margin(1)
    .constraints(
        [
            Constraint::Max(1),
        ]
        .as_ref(),
    )
    .split(popupsize);
    area.render_widget(Clear, popupsize);
    area.render_widget(popup_block, popupsize);
    area.render_widget(input, input_field[0])
}