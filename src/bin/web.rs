//! Web HTML and API
use actix_web::HttpResponse;
use actix_web::{get, post, web, App, HttpServer, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use fc;
use serde::{Deserialize, Serialize};

#[derive(Template)]
#[template(path = "hello.html", print = "all")]
struct HelloTpl<'a> {
    error: &'a str,
    input: &'a Option<fc::T>,
    output: &'a Option<Vec<fc::T>>,
}

#[derive(Serialize, Deserialize)]
struct TFormData {
    value: f32,
    unit: String,
}

#[get("/")]
async fn hello() -> impl Responder {
    HelloTpl {
        error: "",
        input: &None,
        output: &None,
    }
    .into_response()
}

#[post("/")]
async fn bye(params: web::Form<TFormData>) -> impl Responder {
    // TODO: how does Actix Form works -- when there is no unit given??? Because it causes parser error
    match &params.unit[..] {
        "" => HttpResponse::Ok().body("unit is empty string"),
        _ => HttpResponse::Ok().body("blah"),
    };

    match fc::parse_str_to_t(&format!("{}{}", params.value, params.unit)) {
        Ok(t) => {
            let r = fc::convert(t);
            HelloTpl {
                error: "",
                input: &Some(t),
                output: &Some(r),
            }
            .into_response()
        }
        Err(e) => HelloTpl {
            error: e.to_str(),
            input: &None,
            output: &None,
        }
        .into_response(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(hello).service(bye))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
