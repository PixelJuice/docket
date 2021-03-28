use tui::widgets::{ListItem, ListState,List, Block, Borders,};
use tui::style::{Color, Modifier, Style};
use crossterm::{
    event::{read, Event, KeyCode, KeyModifiers},
    Result,
};

use crate::list_util;
use crate::todo_txt::TodoItem;
use crate::todo_txt::TodoTxt;
use crate::Context;

pub fn run(
    contex: &mut Context,
    list_state: &mut ListState,
    todo_txt: &mut TodoTxt,
    quit: &mut bool,
    show_commands: &mut bool,
) -> Result<()> {
    match read()? {
        Event::Key(input_event) => {
            if input_event.modifiers == KeyModifiers::CONTROL
                && input_event.code == KeyCode::Char('c')
                || input_event.code == KeyCode::Char('q')
            {
                *quit = true;
            } else if input_event.modifiers == KeyModifiers::SHIFT
                && input_event.code == KeyCode::Down
            {
                list_util::navigate_list(list_state, &todo_txt.content, 5, true);
            } else if input_event.modifiers == KeyModifiers::CONTROL
                && input_event.code == KeyCode::Down
            {
                list_util::navigate_list(list_state, &todo_txt.content, (&todo_txt.content.len() - 1) as isize, true);
            } else if input_event.code == KeyCode::Down {
                list_util::navigate_list(list_state, &todo_txt.content, 1, false);
            } else if input_event.modifiers == KeyModifiers::SHIFT
                && input_event.code == KeyCode::Up
            {
                list_util::navigate_list(list_state, &todo_txt.content, -5, true);
            } else if input_event.modifiers == KeyModifiers::CONTROL
                && input_event.code == KeyCode::Up
            {
                list_state.select(Some(0));
            } else if input_event.code == KeyCode::Up {
                list_util::navigate_list(list_state, &todo_txt.content, -1, false);
            } else if input_event.code == KeyCode::Char('.') {
                *show_commands = !*show_commands;
            } else if input_event.code == KeyCode::Char('n') {
                    *contex = Context::NewEntry;
            } else if input_event.code == KeyCode::Char(' ') {
                let index = list_state.selected().unwrap();
                todo_txt.toggle(index);
                todo_txt.sort();
            }
        }
        Event::Mouse(_event) => (),
        Event::Resize(_width, _height) => (),
    }
    Ok(())
}

pub fn build_view(todo_txt: &TodoTxt, active: &bool) -> List<'static> {
    let mut items: Vec<ListItem> = Vec::new();
    for item in todo_txt.content.iter() {
        if item.not_done {
            let name = format!(" {}", item.text);
            items.push(ListItem::new(name));
        } else {
            let name = format!(" {}", item.text);
            items.push(ListItem::new(name));
        }
    }
    let mut color = Color::White;
    let mut color_selected = Color::Cyan;
    if !active {
        color = Color::DarkGray;
        color_selected = Color::DarkGray;
    }

    List::new(items.clone())
        .block(Block::default().title("Todo.txt").borders(Borders::ALL))
        .style(Style::default().fg(color))
        .highlight_style(
            Style::default()
                .fg(color_selected)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("ﰲ ")
}
