use actix_web::{http, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use log::*;

async fn serve(req: HttpRequest) -> impl Responder {
    let websocket_addr: String = req.app_data::<String>().unwrap().clone();
    HttpResponse::Ok()
        .cookie(http::Cookie::build("websocket_addr", websocket_addr).finish())
        .content_type("text/html")
        .body(include_str!("./client.html"))
}

async fn serve_script(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/javascript")
        .body(include_str!("./client.js"))
}

pub(crate) async fn start_server(web_addr: String, websocket_addr: String) {
    info!("web server started on {}", web_addr);
    let future = {
        HttpServer::new(move || {
            App::new()
                .app_data(websocket_addr.clone())
                .route("/client.js", web::get().to(serve_script))
                .route("/", web::get().to(serve))
        })
        .bind(web_addr)
        .unwrap()
        .run()
    };
    future.await.expect("http server failed");
    info!("web server stopped");
}
