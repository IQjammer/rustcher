use std::{io, time::Duration};

use crate::{action::Action, desktop, list::List, matcher::FuzzyMatcher, ui};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{DefaultTerminal, Frame};
use ratatui_image::picker::Picker;

pub struct App {
    pub exit: bool,
    pub list: List,
    pub input: String,
    pub picker: Picker,
    matcher: FuzzyMatcher,
}

impl App {
    pub fn new() -> Self {
        let picker = Picker::from_query_stdio().unwrap_or_else(|_| Picker::halfblocks());
        let entries = desktop::load_desktop_entries(&picker);
        Self {
            exit: false,
            list: List::new(entries),
            input: String::new(),
            matcher: FuzzyMatcher::new(),
            picker,
        }
    }

    fn refilter(&mut self) {
        let indices = self.matcher.filter(&self.input, self.list.names());
        self.list.set_visible(indices);
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        ui::render(self, frame.area(), frame.buffer_mut());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        if event::poll(Duration::from_millis(16))? {
            if let Event::Key(key) = event::read()?
                && key.kind == KeyEventKind::Press
            {
                let action = self.map_key_to_action(key);
                self.update(action);
            } else {
                self.update(Action::Tick);
            }
        }
        Ok(())
    }

    fn map_key_to_action(&self, key: KeyEvent) -> Action {
        match (key.modifiers, key.code) {
            (KeyModifiers::NONE, KeyCode::Esc) => Action::Quit,
            (KeyModifiers::NONE, KeyCode::Enter) => Action::Run,
            (KeyModifiers::NONE, KeyCode::Backspace) => Action::RemoveChar,
            (KeyModifiers::CONTROL, KeyCode::Char('j')) => Action::Down,
            (KeyModifiers::CONTROL, KeyCode::Char('k')) => Action::Up,
            (KeyModifiers::NONE, KeyCode::Char(c)) => Action::InsertChar(c),
            _ => Action::None,
        }
    }

    fn update(&mut self, action: Action) {
        match action {
            Action::Quit => self.exit = true,
            Action::Run => {
                if let Some(entry) = self.list.selected() {
                    let _ = entry.launch();
                    self.exit = true;
                }
            }
            Action::Down => self.list.down(),
            Action::Up => self.list.up(),
            Action::InsertChar(c) => {
                self.input.push(c);
                self.refilter();
            }
            Action::RemoveChar => {
                self.input.pop();
                self.refilter();
            }
            Action::Tick | Action::None => {}
        }
    }
}
