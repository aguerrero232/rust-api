use actix_web::{ 
    get, 
    post, 
    web, 
    App, 
    HttpResponse, 
    HttpServer,
    Responder,
    Result,
}; 

use serde::{
    Deserialize, 
    Serialize
};

use std::env;

#[derive(Debug, Serialize, Deserialize)]
struct Data {
    stuff: String,
}

// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Data>) -> Result<String> {
    Ok(format!("Welcome {}!", info.stuff))
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body) 
}

async fn manual_hello() -> impl Responder { 
    HttpResponse::Ok().body("Hey there!") 
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    if env::var("ENV_HOST").is_err() { env::set_var("ENV_HOST", "localhost") }
    let env_host: String = env::var("ENV_HOST").expect("Host must be set");

    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((env_host, 8080))?
    .run()
    .await
}