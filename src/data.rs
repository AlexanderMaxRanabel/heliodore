use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static url: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
