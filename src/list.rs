use freedesktop_desktop_entry::{DesktopEntry, Iter, default_paths};
use ratatui::layout::Size;
use ratatui::widgets::ListState;
use ratatui_image::picker::Picker;
use ratatui_image::protocol::Protocol;
use std::path::PathBuf;

use crate::desktop;

pub struct List {
    pub items: Vec<desktop::EntryList>,
    pub list_state: ListState,
    pub input: String,
}

impl List {
    pub fn new(picker: &Picker) -> Self {
        let mut list_state = ListState::default();

        if !items.is_empty() {
            list_state.select(Some(0));
        }

        Self {
            items: desktop::EntryList::new(),
            list_state,
            input: String::new(),
        }
    }

    pub fn up(&mut self) {
        if !self.items.is_empty() {
            let i = match self.list_state.selected() {
                Some(0) | None => self.items.len() - 1,
                Some(i) => i - 1,
            };
            self.list_state.select(Some(i));
        }
    }

    pub fn down(&mut self) {
        if !self.items.is_empty() {
            let i = match self.list_state.selected() {
                Some(i) if i >= self.items.len() - 1 => 0,
                Some(i) => i + 1,
                None => 0,
            };
            self.list_state.select(Some(i))
        }
    }
}
