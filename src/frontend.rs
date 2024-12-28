use crate::{
    backend,
};

use std::io::{self};

use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Direction, Layout, Rect, Size},
    style::{palette::tailwind, Color, Stylize},
    text::{Line, Text},

    widgets::*,
    DefaultTerminal,
};

use tui_scrollview::{ScrollView, ScrollViewState};

#[derive(Debug, Default, Clone)]
pub struct App {
    text: String,
    scroll_view_state: ScrollViewState,
    state: AppState,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quit,
}

impl App {
    pub async fn new(mut url: String) -> App {
        let content: String = backend::mk_req(url).await.expect("Async shat itself");
        App {   
            text: content,
            ..Default::default()
        }
    }

    pub fn run(&mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while self.is_running() {
            self.draw(&mut terminal)?;
            self.handle_events()?;
        }
        Ok(())

    }

    pub fn is_running(&self) -> bool {
        self.state == AppState::Running
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quit;
    }

    pub fn draw(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        terminal.draw(|frame| {
            let area = frame.area();
            let size = Size::new(area.width * 2, area.width);
            let mut scroll_view = ScrollView::new(size);
            let paragraph = Paragraph::new(self.text.clone()).wrap(Wrap::default());
            scroll_view.render_widget(paragraph, scroll_view.area());
            frame.render_stateful_widget(scroll_view, area, &mut self.scroll_view_state);

        })?;
        Ok(())
    }

    pub fn handle_events(&mut self) -> anyhow::Result<()> {
        use KeyCode::*;
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Char('h') | Left => self.scroll_view_state.scroll_left(),
                Char('l') | Right => self.scroll_view_state.scroll_right(),
                Char('j') | Up => self.scroll_view_state.scroll_page_up(),
                Char('k') | Down => self.scroll_view_state.scroll_page_down(),
                _ => (),
            },
            _ => {}
        }
        Ok(())
    }
}
