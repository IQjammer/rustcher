use ratatui::widgets::ListState;

use crate::desktop::AppEntry;

pub struct List {
    entries: Vec<AppEntry>,
    pub list_state: ListState,
    visible: Vec<usize>,
}

impl List {
    pub fn new(mut entries: Vec<AppEntry>) -> Self {
        entries.sort_by_key(|e| e.name.to_lowercase());
        let visible = (0..entries.len()).collect();
        let mut list = Self {
            entries,
            visible,
            list_state: ListState::default(),
        };
        list.reset_selection();
        list
    }

    pub fn names(&self) -> impl Iterator<Item = &str> {
        self.entries.iter().map(|e| e.name.as_str())
    }

    pub fn set_visible(&mut self, indices: Vec<usize>) {
        self.visible = indices;
        self.reset_selection();
    }

    pub fn visible(&self) -> impl Iterator<Item = &AppEntry> {
        self.visible.iter().map(|&i| &self.entries[i])
    }

    pub fn selected(&self) -> Option<&AppEntry> {
        let i = self.list_state.selected()?;
        self.visible.get(i).map(|&idx| &self.entries[idx])
    }

    fn reset_selection(&mut self) {
        self.list_state.select(if self.visible.is_empty() {
            None
        } else {
            Some(0)
        });
    }

    pub fn up(&mut self) {
        if self.visible.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(0) | None => self.visible.len() - 1,
            Some(i) => i - 1,
        };
        self.list_state.select(Some(i));
    }

    pub fn down(&mut self) {
        if self.visible.is_empty() {
            return;
        }
        let i = match self.list_state.selected() {
            Some(i) if i + 1 < self.visible.len() => i + 1,
            _ => 0,
        };
        self.list_state.select(Some(i));
    }
}
