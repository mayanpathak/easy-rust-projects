// use actix_web::{web, App, HttpRequest, HttpServer, HttpResponse, Responder};

// async fn greet(req: HttpRequest)-> impl Responder{
//     let name: &str = req.match_info().get(name: "name").unwraap_or(default: "world");

// }


// async fn health_check(req: HttpRequest) -> impl Responder {
//     HttpResponse::Ok().body("OK")
// }




// #[tokio::main]

// async fn main ()-> std ::io::Ressult<()> {
//     HttpServer::new (||{
//         App::new() App<AppEntry>
//         .route(path: "/hello/{name}", route:web::get().to(handler: greet))App<AppEntry>
//         .route (path: "/hello", route:web::get().to(handler:greet))App<AppEntry>
//        .route( path: "/health_check", route:web::get().to(handler: health_check))
//     })

//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
// }
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};

async fn greet(req: HttpRequest) -> impl Responder {
    // Get "name" from URL or default to "world"
    let name = req.match_info().get("name").unwrap_or("world");
    HttpResponse::Ok().body(format!("Hello {name}!"))
}

async fn health_check() -> impl Responder {
    HttpResponse::Ok().body("OK")
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello/{name}", web::get().to(greet))
            .route("/hello", web::get().to(greet))
            .route("/health_check", web::get().to(health_check))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

