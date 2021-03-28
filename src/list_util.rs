use crate::TodoItem;
use tui::widgets::{ListItem, ListState};

pub fn navigate_list(list_state: &mut ListState, list: &Vec<TodoItem>, step: isize, clamp: bool) {
    let mut new_index = list_state.selected().unwrap() as isize + step;
    let list_len = list.len() as isize;
    if clamp {
        new_index = isize::clamp(new_index, 0, list_len - 1)
    }
    let wrapped_index = ((new_index % list_len + list_len) % list_len) as usize;
    list_state.select(Some(wrapped_index as usize))
}

#[test]
fn test_negative_wrapping() {
    let mut items: Vec<TodoItem> = Vec::new();
    for _x in 0..10 {
        let name = String::from("Item");
        items.push(TodoItem {
            text: name,
            not_done: false,
            priority: ' ',
        });
    }
    let mut list_state = ListState::default();
    list_state.select(Some(0));
    navigate_list(&mut list_state, &items, -1, false);
    assert_eq!(list_state.selected().unwrap(), 9);
}

#[test]
fn test_positive_wrapping() {
    let mut items: Vec<TodoItem> = Vec::new();
    for _x in 0..10 {
        let name = String::from("Item");
        items.push(TodoItem {
            text: name,
            not_done: false,
            priority: ' ',
        });
    }
    let mut list_state = ListState::default();
    list_state.select(Some(9));
    navigate_list(&mut list_state, &items, 1, false);
    assert_eq!(list_state.selected().unwrap(), 0);
}

#[test]
fn test_next() {
    let mut items: Vec<TodoItem> = Vec::new();
    for _x in 0..10 {
        let name = String::from("Item");
        items.push(TodoItem {
            text: name,
            not_done: false,
            priority: ' ',
        });
    }
    let mut list_state = ListState::default();
    list_state.select(Some(5));
    navigate_list(&mut list_state, &items, 1, false);
    assert_eq!(list_state.selected().unwrap(), 6);
}

#[test]
fn test_previous() {
    let mut items: Vec<TodoItem> = Vec::new();
    for _x in 0..10 {
        let name = String::from("Item");
        items.push(TodoItem {
            text: name,
            not_done: false,
            priority: ' ',
        });
    }
    let mut list_state = ListState::default();
    list_state.select(Some(5));
    navigate_list(&mut list_state, &items, -1, false);
    assert_eq!(list_state.selected().unwrap(), 4);
}

#[test]
fn test_clamp_zero() {
    let mut items: Vec<TodoItem> = Vec::new();
    for _x in 0..10 {
        let name = String::from("Item");
        items.push(TodoItem {
            text: name,
            not_done: false,
            priority: ' ',
        });
    }
    let mut list_state = ListState::default();
    list_state.select(Some(5));
    navigate_list(&mut list_state, &items, -10, true);
    assert_eq!(list_state.selected().unwrap(), 0);
}

#[test]
fn test_clamp_max() {
    let mut items: Vec<TodoItem> = Vec::new();
    for _x in 0..10 {
        let name = String::from("Item");
        items.push(TodoItem {
            text: name,
            not_done: false,
            priority: ' ',
        });
    }
    let mut list_state = ListState::default();
    list_state.select(Some(5));
    navigate_list(&mut list_state, &items, 10, true);
    assert_eq!(list_state.selected().unwrap(), 9);
}
