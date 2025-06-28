use actix_web::{get, App, HttpResponse, HttpServer, Responder};
use config::Config;
use log::info;
use prometheus::{register_counter, Counter, Encoder, TextEncoder};
use serde::Deserialize;
use std::sync::LazyLock;

static REQUEST_COUNTER: LazyLock<Counter> =
    LazyLock::new(|| register_counter!("request_count", "Nombre de requêtes reçues").unwrap());

#[derive(Deserialize)]
struct Settings {
    server_host: String,
    server_port: u16,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json("OK")
}

#[get("/metrics")]
async fn metrics() -> impl Responder {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();
    encoder.encode(&metric_families, &mut buffer).unwrap();
    HttpResponse::Ok()
        .content_type(encoder.format_type())
        .body(buffer)
}

#[get("/")]
async fn index() -> impl Responder {
    REQUEST_COUNTER.inc();
    HttpResponse::Ok().json("Hello from podia-filtering-service")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    let settings = Config::builder()
        .add_source(config::File::with_name("Config"))
        .add_source(config::Environment::default())
        .build()
        .unwrap()
        .try_deserialize::<Settings>()
        .unwrap();

    let addr = format!("{}:{}", settings.server_host, settings.server_port);
    info!("Starting server at {addr}");
    HttpServer::new(|| App::new().service(health).service(metrics).service(index))
        .bind(addr)?
        .run()
        .await
}
