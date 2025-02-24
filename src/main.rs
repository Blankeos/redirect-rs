use actix_web::{self as actix, HttpRequest};

#[derive(serde::Serialize)]
struct IndexResponse {
    message: String,
    examples: Vec<String>,
}

#[actix::get("/")]
async fn index(req: actix::HttpRequest) -> impl actix::Responder {
    let protocol = req.connection_info().scheme().to_string();
    let host = req.connection_info().host().to_string();

    let base_url = format!("{}://{}", protocol, host);

    actix::web::Json(IndexResponse {
        message: format!("To get started, pass {}/{{your url here}}", base_url),
        examples: vec![
            format!("{}/https://carlo.vercel.app", base_url),
            format!("{}/figma.com", base_url),
        ],
    })
}

#[actix::get("/{url:.*}")]
async fn redirect(path: actix::web::Path<String>, req: HttpRequest) -> impl actix::Responder {
    let redirect_url = path.into_inner();

    if redirect_url.is_empty() {
        return actix::HttpResponse::BadRequest().body("URL cannot be empty");
    }

    let query_string = if let Some(query) = req.uri().query() {
        format!("?{}", query)
    } else {
        String::new()
    };

    let final_url = format!("{}{}", redirect_url, query_string);

    println!("Redirecting to: {}", final_url);

    actix::HttpResponse::TemporaryRedirect()
        .append_header(("Location", final_url))
        .finish()
}

#[actix::main]
async fn main() -> std::io::Result<()> {
    println!("Server starting at http://127.0.0.1:8080");

    actix::HttpServer::new(|| actix::App::new().service(index).service(redirect))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
