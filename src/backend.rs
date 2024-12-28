use crate::data::url;
use trotter::{Actor, UserAgent};

pub async fn make_request_gemini(mut gemini_url: String) -> anyhow::Result<String> {
    if !gemini_url.ends_with("/") {
        gemini_url = format!("{}/", gemini_url);
    }

    let requester = Actor::default().user_agent(UserAgent::Archiver);

    let response = requester.get(gemini_url).await?.gemtext()?;

    Ok(response)
}

pub fn set_string(new_value: String) {
    let mut data = url.lock().unwrap();
    *data = new_value;
}
