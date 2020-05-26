use http_types::{Method, Request, Response, StatusCode, Url};
use smol::block_on as smol_block_on;
use std::process::Command;
use std::str::FromStr;

pub fn http_client(
    method: String,
    url: String,
    token: Option<String>,
    body_string: Option<String>,
) -> Result<String, String> {
    let parsed_method = Method::from_str(&method).unwrap();
    let mut req = Request::new(parsed_method, Url::parse(&url).unwrap());
    let body_string = body_string.unwrap_or_else(|| "".to_string());
    req.set_body(body_string);
    smol_block_on(async {
        match curl(req, token).await {
            Ok(mut response) => {
                let body_string = response.body_string().await.unwrap();
                println!("{}", &body_string);
                Ok(body_string)
            }
            Err(mut response) => {
                let body_string = response.body_string().await.unwrap();
                eprintln!("ERROR:");
                dbg!(&body_string);
                Err(body_string)
            }
        }
    })
}

async fn curl(mut req: Request, token: Option<String>) -> Result<Response, Response> {
    let url = req.url().clone();
    let user_agent = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    let method = req.method().to_string();
    let body = req.body_string().await.unwrap();
    let token = token.unwrap_or_else(|| "".to_string());
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
        let mut response = Response::new(StatusCode::Ok);
        response.set_body(stdout);
        Ok(response)
    } else {
        let stderr = command.stderr.to_vec();
        let stderr = std::str::from_utf8(&stderr).unwrap().to_string();
        let status_code = match command.status.code().unwrap() {
            22 => StatusCode::NotFound,
            7 => StatusCode::ExpectationFailed,
            _ => StatusCode::Conflict,
        };
        let mut response = Response::new(status_code);
        response.set_body(stderr);
        Err(response)
    }
}
