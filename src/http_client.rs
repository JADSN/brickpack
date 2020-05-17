use http_types::{Request, Response, StatusCode};
use std::process::Command;

async fn curl(req: Request, token: Option<String>) -> Result<Response, Response> {
    let url = req.url().clone();
    let body_json = req.body_string().await.unwrap();
    let token = token.unwrap_or_else(|| "".to_string());
    // r#"curl -f --request POST \
    let cmd_line = format!(
        r#"curl -f --request GET \
        --header 'content-type: application/json' \
        --header 'authorization: Bearer {}' \
    --url {} \
    --data '{}'"#,
        token, url, body_json
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

pub fn http_client(
    url: String,
    token: Option<String>,
    body_string: Option<String>,
) -> Result<String, String> {
    use async_std::task;
    use http_types::{Method, Url};

    task::block_on(async {
        let mut req = Request::new(Method::Get, Url::parse(&url).unwrap());
        let body_string = body_string.unwrap_or_else(|| "".to_string());
        req.set_body(body_string);
        match curl(req, token).await {
            Ok(response) => {
                let body_string = response.body_string().await.unwrap();
                println!("{}", &body_string);
                Ok(body_string)
            }
            Err(response) => {
                let body_string = response.body_string().await.unwrap();
                eprintln!("ERROR:");
                dbg!(&body_string);
                Err(body_string)
            }
        }
    })
}
