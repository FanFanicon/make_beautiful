
use actix_web::{middleware, web, App, HttpRequest, HttpServer,get,Responder,Either, http::{
        header::{self, ContentType},
        Method, StatusCode,
    },HttpResponse,Result};
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_files;



async fn react_index(req_method:Method) ->Result<impl Responder>{
     match req_method{   //匹配当前的请求类型
        Method::GET=>{  //Method是一个枚举类型,Get方法的时候这样子做
            let file=actix_files::NamedFile::open("dist/index.html")?  //打开404htmo
                .customize()  //该函数包装响应对象,返回一个允许改变的响应对象
                .with_status(StatusCode::ACCEPTED);   //带上响应码
            Ok(Either::Left(file))
        }
        _=>Ok(Either::Right(HttpResponse::MethodNotAllowed().finish())) //其他的请求类型就会返回不被允许类型的状态码
    }
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
            .default_service( //解决前端路由刷新页面404问题
                web::get().to(react_index)
            )
            
            
            
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}