use crate::models::WorldState;

const API_URL: &str = "https://api.warframestat.us/pc/?language=en";

pub async fn refresh_connection() -> Result<WorldState, reqwest::Error> {
    let response = reqwest::get(API_URL).await?;

    let text = response.text().await?;

    match serde_json::from_str::<WorldState>(&text) {
        Ok(world) => Ok(world),
        Err(err) => {
            println!("Serde error:\n{err}");
            panic!();
        }
    }
}
