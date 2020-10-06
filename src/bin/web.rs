//! Web HTML and API
use actix_web::{get, post, web, App, HttpServer, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use serde::{Deserialize, Serialize};
use fc;

#[derive(Template)]
#[template(path="hello.html")]
struct HelloTpl<'a> {
    error: &'a str,
    result: &'a str,
}

#[derive(Serialize, Deserialize)]
struct RawT {
    value: f32,
    unit: String
}

#[get("/")]
async fn hello() -> impl Responder {
    HelloTpl { error: "", result: "" }.into_response()
}

#[post("/")]
async fn result(params: web::Form<RawT>) -> impl Responder {
    match fc::parse_str_to_t(&format!("{}{}", params.value, params.unit)) {
        Ok(t) => {
            let r = fc::convert(t);
            HelloTpl { error: "", result: "blah".into() }.into_response()
        },
        Err(e) => HelloTpl { error: e.to_str(), result: "" }.into_response()
    };
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
