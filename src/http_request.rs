use reqwest::StatusCode;

pub(crate) async fn get_status(api_url: &str) -> Result<StatusCode, String> {
    let client = match reqwest::Client::builder()
        .user_agent("Farid the agent")
        .build()
    {
        Ok(c) => c,
        Err(e) => {
            return Err(e.to_string());
        }
    };

    match client.get(api_url).send().await {
        Ok(r) => Ok(r.status()),
        Err(e) => Err(e.to_string()),
    }
}
