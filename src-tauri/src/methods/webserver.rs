use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MyData {
    name: String,
}

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, world!")
}

#[post("/submit")]
async fn submit(data: web::Json<MyData>) -> impl Responder {
    println!("Received data: {:?}", data);
    HttpResponse::Ok().body("Data received")
}

pub async fn run_server() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(Cors::permissive()) // 允许跨域请求
            .service(index)
            .service(submit)
    })
    .bind("127.0.0.1:18777")?
    .run()
    .await
}
