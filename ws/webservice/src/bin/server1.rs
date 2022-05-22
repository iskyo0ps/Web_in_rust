use actix_web::{web, App, HttpResponse,HttpServer, Responder};
use std::io;

// config route
pub fn general_routes(cfg:&mut web:: ServiceConfig){
  cfg.route("/health",web::get().to(health_check_handler));
}

// config health_check_handler
pub async fn health_check_handler() -> impl Responder {
  HttpResponse::Ok().json("git is a good thing!")
}

// instantiate Http server and running
#[actix_rt::main]
async fn main() -> io::Result<()> {
  // construct app, config router
  let app = move || App::new().configure(general_routes);

  // runing HTTP Server
  HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}