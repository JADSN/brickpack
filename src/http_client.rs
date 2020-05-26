use reqwest::header::{AUTHORIZATION, USER_AGENT};

pub fn http_client(
    method: String,
    url: String,
    token: Option<String>,
    body_string: Option<String>,
) -> Result<String, String> {
    let client = reqwest::blocking::Client::new();
    // * Set Method
    let request = match method.as_ref() {
        "POST" => client.post(&url),
        "PATCH" => client.patch(&url),
        "PUT" => client.put(&url),
        "DELETE" => client.delete(&url),
        _ => client.get(&url),
    };
    // * Set User Agent
    let user_agent = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let request = request.header(USER_AGENT, user_agent);

    // * Set Token Authentication
    let token = token.unwrap_or_else(|| "".to_string());
    let bearer_token = format!("Bearer {}", token);
    let request = request.header(AUTHORIZATION, bearer_token);

    // * Set Body and Send
    let body = body_string.unwrap_or_else(|| "".to_string());
    let response = match request.body(body).send() {
        Ok(response) => response.text().unwrap_or_else(|error| error.to_string()),
        Err(error) => error.to_string(),
    };

    Ok(response)
}
