use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
 
 async fn greet (req: HttpRequest)-> impl Responder{
    let name = req.match_info().get("name").unwrap_or("world");
    HttpResponse::Ok().body(format!("Hello {name}!"))

 }


 async fn health_check()-> impl Responder{
    HttpResponse::Ok().body("ok")
 }


 #[tokio::main]

 async fn main ()-> std:: io:: Result<()>{
    HttpServer::new(||{
        App::new()
        .route("/hello/{name}", web::get().to(greet))
        .route("/hello", web :: get().to(greet))
        .route("/health_check", web::get().to(health_check))

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
 }