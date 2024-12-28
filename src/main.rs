mod frontend;
mod backend;
mod data;

use std::{env, io::{self}};

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

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let terminal = ratatui::init();
    let args: Vec<String> = env::args().collect();
    let mut url_normal = args.get(1).unwrap_or_else(|| {
        println!("No url was provided");
        std::process::exit(1);
    });

    backend::set_string(url_normal.to_string());    

    let result = frontend::App::new().await.run(terminal); 
    ratatui::restore();
    result
}
