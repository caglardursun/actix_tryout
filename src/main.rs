mod models;
use models::Status;
mod config;

use actix_web::{web, App, Responder, HttpServer};
use dotenv::dotenv;

async fn status() -> impl Responder {
    web::HttpResponse::Ok()
    .json(Status {status : "Ok".to_string() } )

}


#[actix_rt::main]
async fn main() -> std::io::Result<()>{

    dotenv().ok();
    
    let conf = config::Config::from_env().unwrap();
    let url: String = format!("{}:{}",conf.server.host,conf.server.port);
     
    println!("Server is starting on {}",url);

    HttpServer::new(move || {
        App::new()
        .route("/", web::get().to(status))
            
    })
    .bind(url)?    
    //.workers(1)
    .run()
    .await
}
