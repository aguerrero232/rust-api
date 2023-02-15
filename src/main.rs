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

#[derive(Serialize, Deserialize)]
struct Info {
    username: String,
}

// deserialize `Info` from request's body
#[post("/submit")]
async fn submit(info: web::Json<Data>) -> Result<String> {
    Ok(format!("Welcome {}!", info.stuff))
}

// extract info using serde
#[post("/index")]
async fn index(info: web::Json<Info>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body) 
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
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
            .service(submit)
            .service(index)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind((env_host, 8080))?
    .run()
    .await
}