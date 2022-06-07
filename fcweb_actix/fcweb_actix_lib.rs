use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use askama_actix::{Template, TemplateToResponse};
use fc_lib;
//use openssl::ssl::{SslAcceptor, SslFiletype, SslMethod};
use serde::{Deserialize, Serialize};
use std::net;

#[derive(Template)]
#[template(path = "hello.html", print = "none")]
pub struct HelloTpl<'a> {
    error: &'a str,
    input: &'a Option<fc_lib::T>,
    output: &'a Option<Vec<fc_lib::T>>,
}

#[derive(Serialize, Deserialize)]
pub struct TFormData {
    value: f32,
    unit: Option<String>,
}

pub async fn hello() -> HttpResponse {
    HelloTpl {
        error: "",
        input: &None,
        output: &None,
    }.to_response()
}

pub async fn bye(params: web::Form<TFormData>) -> HttpResponse {
    match fc_lib::parse_str_to_t(&format!(
        "{}{}",
        params.value,
        params.unit.as_ref().unwrap_or(&String::from(""))
    )) {
        Ok(t) => {
            let r = fc_lib::convert(t);
            HelloTpl {
                error: "",
                input: &Some(t),
                output: &Some(r),
            }
            .to_response()
        }
        Err(e) => HelloTpl {
            error: e.to_str(),
            input: &None,
            output: &None,
        }
        .to_response(),
    }
}

#[actix_web::main]
pub async fn webmain(ipaddr_n_port: net::SocketAddrV4) -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    // let mut ssl_builder = SslAcceptor::mozilla_intermediate(SslMethod::tls()).unwrap();
    // ssl_builder.set_private_key_file("/home/likwoka/keys/testkey.pem", SslFiletype::PEM).unwrap();
    // ssl_builder.set_certificate_chain_file("/home/likwoka/keys/testcert.pem").unwrap();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::DefaultHeaders::new().add(("permissions-policy", "interest-cohort=()")))
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hello))
            .route("/", web::post().to(bye))
    })
    .bind(ipaddr_n_port)?
    // .bind_openssl("127.0.0.1:8443", ssl_builder)?
    .run()
    .await
}
