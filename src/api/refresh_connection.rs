use crate::models::WorldState;

const API_URL: &str = "https://api.warframestat.us/pc/?language=en";

pub async fn refresh_connection() -> Result<WorldState, reqwest::Error> {
    let world_state = reqwest::get(API_URL).await?.json::<WorldState>().await?;

    Ok(world_state)
}
