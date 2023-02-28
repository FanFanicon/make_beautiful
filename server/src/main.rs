use std::ffi::c_long;

use actix_web::{middleware, web, App, HttpRequest, HttpServer,get, http::{
        header::{self, ContentType},
        Method, StatusCode,
    },HttpResponse,Result};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_files;



async fn react_index() -> HttpResponse{
    println!("docs");
   HttpResponse::Ok().content_type("application/json").body("我尼玛docs")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            .service(actix_files::Files::new("/","./dist").index_file("index.html")) //处理静态文件
            .default_service(
                web::get().to(react_index)
            )
            
            
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}