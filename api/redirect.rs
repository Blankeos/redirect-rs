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
use querystring::{self, querify};
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

    match query_string {
        Some(qs) => {
            let queries: std::collections::HashMap<_, _> = querify(qs).into_iter().collect();

            // From Vercel, because vercel encodes it before passing it as a querystring.
            let encoded_url = queries.get("redirect_url").unwrap_or(&"");

            // Decode it again, so it looks similar to what was first passed.
            let redirect_url =
                urlencoding::decode(encoded_url).unwrap_or_else(|_| encoded_url.to_string().into());

            // Add protocol if missing (because Vercel removes some slashes after the encoding process).
            let final_url = if !redirect_url.contains("://") {
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
            // println!("carlo we are here, let's redirect.");
            // return Ok(Response::builder()
            //     .status(StatusCode::OK)
            //     .header("Content-Type", "application/json")
            //     .body(
            //         json!({
            //             "message": "Redirecting...",
            //             "redirect_url": final_url
            //         })
            //         .to_string()
            //         .into(),
            //     )?);
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
