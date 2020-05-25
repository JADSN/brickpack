use std::process::Command;
// use std::str::FromStr;

pub fn http_client(
    method: String,
    url: String,
    token: Option<String>,
    body_string: Option<String>,
) -> Result<String, String> {
    let user_agent = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let token = token.unwrap_or_else(|| "".to_string());
    let body = body_string.unwrap_or_else(|| "".to_string());
    let cmd_line = format!(
        r#"curl -f --request {method} \
        --header 'User-Agent: {user_agent}' \
        --header 'Content-Type: application/json' \
        --header 'Authorization: Bearer {token}' \
    --url {url} \
    --data '{body}'"#,
        user_agent = user_agent,
        method = method,
        token = token,
        url = url,
        body = body
    );
    let command = Command::new("sh")
        .arg("-c")
        .arg(cmd_line)
        .output()
        .expect("failed to execute process");

    if command.status.success() {
        let stdout = command.stdout.to_vec();
        let stdout = std::str::from_utf8(&stdout).unwrap().to_string();
        Ok(stdout)
    } else {
        let stderr = command.stderr.to_vec();
        let stderr = std::str::from_utf8(&stderr).unwrap().to_string();
        Err(stderr)
    }
}
