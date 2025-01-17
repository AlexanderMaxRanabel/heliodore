use once_cell::sync::Lazy;
use std::sync::Mutex;

pub static URL: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static CONTENT: Lazy<Mutex<String>> = Lazy::new(|| Mutex::new(String::new()));
pub static URL_CHANGE: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new("True"));
