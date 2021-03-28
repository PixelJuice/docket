use crossterm::Result;
use std::io;
use tui::backend::CrosstermBackend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Modifier, Style};

use tui::widgets::{Block, Borders, List, ListItem, ListState, Paragraph,BorderType};
use tui::Terminal;

mod app;
mod list_util;
mod list_view;
mod todo_txt;
mod help_view;
mod new_view;

use todo_txt::TodoItem;
use todo_txt::TodoTxt;

#[derive(PartialEq)]
pub enum Context {
    NewEntry,
    EditEntry,
    ViewList,
}

fn main() -> Result<()> {
    let mut terminal = app::start_application().expect("Terminal could not be started");
    let todo_name = String::from("Todo");
    let mut todo_txt = TodoTxt::new(&todo_name);
    let mut quit = false;
    let mut show_commands = false;
    let mut context = Context::ViewList;
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    let mut desc =String::from("");
    while !quit {
        match context {
            Context::NewEntry => new_view::run(
                &mut context,
                &mut desc,
                &mut todo_txt,
                &mut quit,
            )?,
            Context::EditEntry => (),
            Context::ViewList => list_view::run(
                &mut context,
                &mut list_state,
                &mut todo_txt,
                &mut quit,
                &mut show_commands,
            )?,
        }

        render_view(&mut terminal, &mut context, &mut list_state, &todo_txt, &show_commands, &desc)?;
    }
    todo_txt.save_to_disk();
    terminal.show_cursor()?;
    app::close_application()?;
    Ok(())
}

fn render_view(
    terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    context: &mut Context,
    list_state: &mut ListState,
    todo_txt: &TodoTxt,
    show_commands: &bool,
    desc: &String
) -> Result<()> {

    let todo_list: List;
    let menu: Paragraph;
    match context {
        Context::ViewList => {
            todo_list = list_view::build_view(todo_txt, &true);
            menu = help_view::build_view(&true);
        },
        Context::NewEntry => {
            todo_list = list_view::build_view(todo_txt, &false);
            menu = help_view::build_view(&false);
        }
        Context::EditEntry => {
            todo_list = list_view::build_view(todo_txt, &false);
            menu = help_view::build_view(&false);
        }
    }
    let mut command_size = 5;
    if !*show_commands {
        command_size = 1;
    }

    terminal.draw(|f| {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Min(5),
                    Constraint::Length(command_size as u16),
                ]
                .as_ref(),
            )
            .split(f.size());

        f.render_stateful_widget(todo_list, chunks[0], list_state);
        f.render_widget(menu, chunks[1]);
    if *context == Context::NewEntry {
        new_view::render(f, desc);
    }
        
    })?;

    Ok(())
}
