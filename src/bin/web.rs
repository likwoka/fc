//! Web HTML and API
use actix_web::{get, post, web, App, HttpServer, Responder};
use actix_web::HttpResponse;
use askama_actix::{Template, TemplateIntoResponse};
use serde::{Deserialize, Serialize};
use fc;

#[derive(Template)]
#[template(path="hello.html", print = "all")]
struct HelloTpl<'a> {
    error: &'a str,
    input: &'a Option<fc::T>,
    output: &'a Option<Vec<fc::T>>,
}

#[derive(Serialize, Deserialize)]
struct RawT {
    value: f32,
    unit: String
}

#[get("/")]
async fn hello() -> impl Responder {
    HelloTpl { error: "", input: &None, output: &None }.into_response()
}

#[post("/")]
async fn bye(params: web::Form<RawT>) -> impl Responder {
    // TODO: how does Actix Form works -- when there is no unit given??? Because it causes parser error
    match &params.unit[..] {
        "" => HttpResponse::Ok().body("unit is empty string"),
        _ => HttpResponse::Ok().body("blah"),
    };
    match fc::parse_str_to_t(&format!("{}{}", params.value, params.unit)) {
        Ok(t) => {
            HttpResponse::Ok().body(format!("{:?}", t))
            //let r = fc::convert(t);
            //HelloTpl { error: "", input: &None, output: &None }.into_response()
        },
        _ => HttpResponse::Ok().body("blah 333"),
        //Err(e) => HelloTpl { error: e.to_str(), input: &None, output: &None }.into_response()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(bye)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
