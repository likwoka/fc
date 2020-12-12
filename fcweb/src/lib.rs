use actix_web::{web, App, HttpServer, Responder};
use askama_actix::{Template, TemplateIntoResponse};
use fc;
use serde::{Deserialize, Serialize};
use std::net;

#[derive(Template)]
#[template(path = "hello.html", print = "none")]
pub struct HelloTpl<'a> {
    error: &'a str,
    input: &'a Option<fc::T>,
    output: &'a Option<Vec<fc::T>>,
}

#[derive(Serialize, Deserialize)]
pub struct TFormData {
    value: f32,
    unit: Option<String>,
}

pub async fn hello() -> impl Responder {
    HelloTpl {
        error: "",
        input: &None,
        output: &None,
    }
    .into_response()
}

pub async fn bye(params: web::Form<TFormData>) -> impl Responder {
    match fc::parse_str_to_t(&format!(
        "{}{}",
        params.value,
        params.unit.as_ref().unwrap_or(&String::from(""))
    )) {
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
pub async fn webmain(ipaddr_n_port: net::SocketAddrV4) -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/", web::post().to(bye))
    })
    .bind(ipaddr_n_port)?
    .run()
    .await
}
