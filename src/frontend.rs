use crate::backend;
use crate::data::*;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::Size,
    widgets::*,
    DefaultTerminal,
};

use std::io::{self};
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
    pub async fn new() -> App {
        let mut data_url = URL.lock().unwrap();
        let mut data_content = CONTENT.lock().unwrap();
        *data_content = backend::make_request_gemini(data_url.clone()).await.expect("If you are seeing this error, there is likely an issue with frontend communicating with the backend");
        return App {
            text: data_content, 
            ..Default::default()
        }
    }

    pub async fn run(&mut self, mut terminal: DefaultTerminal) -> anyhow::Result<()> {
        while self.is_running() {
            self.draw(&mut terminal)?;
            self.handle_events().await?;
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

    pub async fn handle_events(&mut self) -> anyhow::Result<()> {
        use KeyCode::*;

        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => match key.code {
                Char('q') | Esc => self.quit(),
                Char('h') | Left => self.scroll_view_state.scroll_left(),
                Char('l') | Right => self.scroll_view_state.scroll_right(),

                Char('j') | Up => self.scroll_view_state.scroll_page_up(),
                Char('k') | Down => self.scroll_view_state.scroll_page_down(),
                Char('n') => {
                    let mut local_url = String::new();
                    while let Event::Key(KeyEvent { code, .. }) = event::read()? {
                        match code {
                            KeyCode::Enter => {
                                break;
                            }
                            KeyCode::Char(c) => {
                                local_url.push(c);
                            }
                            _ => {}
                        }
                    }

                    backend::set_string_wrapper_url(local_url);
                    let data_url = URL.lock().unwrap();
                    let mut data_content = CONTENT.lock().unwrap();
                    *data_content = backend::make_request_gemini(data_url.clone()).await.expect("If you are seeing this error, there is likely an issue with frontend communicating with the backend");
                }
                _ => (),
            },

            _ => {}
        }

        Ok(())
    }
}
