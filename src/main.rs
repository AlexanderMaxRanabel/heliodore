mod backend;
mod data;
mod frontend;

use std::env;



use crate::data::*;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let terminal = ratatui::init();
    let args: Vec<String> = env::args().collect();
    let url_normal = args.get(1).unwrap_or_else(|| {
        println!("No url was provided");
        std::process::exit(1);
    });

    backend::set_string_wrapper_url(url_normal.to_string());

    let data_url = URL.lock().unwrap();
    let result = frontend::App::new().await.run(terminal).await;
    ratatui::restore();
    result
}
