// use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};

// #[tokio::main]
// async fn main() -> Result<(), Error> {
//     run(handler).await
// }

// pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
//     Ok(Response::builder()
//         .status(StatusCode::TEMPORARY_REDIRECT)
//         .header("Location", "/https://carlotaleon.net")
//         .body(Body::Empty)?)
// }

// Easy temp for testing:
use querystring::{self, querify, stringify};
use serde_json::json;
use vercel_runtime::{Body, Error, Request, Response, StatusCode, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(req: Request) -> Result<Response<Body>, Error> {
    let query_string = match req.uri().query() {
        Some("redirect_url=") => None,
        other => other,
    };

    println!("detected query string: {:?}", query_string);

    match query_string {
        Some(qs) => {
            let queries = querify(qs);

            let mut redirect_url = String::new();
            let mut redirect_queries: Vec<(&str, &str)> = Vec::new();

            for (key, value) in queries {
                // From Vercel, because vercel encodes it before passing it as a querystring.
                if key.eq("redirect_url") {
                    let encoded_url = value;
                    // Decode it again, so it looks similar to what was first passed.
                    redirect_url = urlencoding::decode(encoded_url)
                        .unwrap_or_else(|_| encoded_url.to_string().into())
                        .into_owned();

                    // Add protocol if missing (because Vercel removes some slashes after the encoding process).
                    redirect_url = if !redirect_url.contains("://") {
                        if redirect_url.starts_with("https:/") {
                            redirect_url.replacen("https:/", "https://", 1)
                        } else if redirect_url.starts_with("http:/") {
                            redirect_url.replacen("http:/", "http://", 1)
                        } else {
                            format!("https://{}", redirect_url)
                        }
                    } else {
                        redirect_url.to_string()
                    };
                } else {
                    redirect_queries.push((key, value));
                }
            }

            let stringified_redirect_queries = stringify(redirect_queries);

            let final_url = if stringified_redirect_queries.is_empty() {
                redirect_url
            } else {
                format!("{}?{}", redirect_url, stringified_redirect_queries)
            };

            println!("redirecting to {}", final_url);
            return Ok(Response::builder()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .header("Location", final_url)
                .body(Body::Empty)?);
        }
        None => {
            let protocol = req.uri().scheme_str().unwrap_or("http");
            let host = req.uri().host().unwrap_or("localhost");
            let port = req
                .uri()
                .port()
                .map(|p| p.to_string())
                .unwrap_or(String::from(""));

            let origin = format!("{}://{}:{}", protocol, host, port);

            return Ok(Response::builder()
                .status(StatusCode::OK)
                .header("Content-Type", "application/json")
                .body(
                    json!({
                        "message": format!("To get started, pass something like {}/<your-url>", origin),
                        "examples": vec![
                            format!("{}/https://carlotaleon.net", origin),
                            format!("{}/figma.com", origin)
                        ],
                    })
                    .to_string()
                    .into(),
                )?);
        }
    }
}
