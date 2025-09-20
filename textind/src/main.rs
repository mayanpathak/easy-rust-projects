// use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
 
//  async fn greet (req: HttpRequest)-> impl Responder{
//     let name = req.match_info().get("name").unwrap_or("world");
//     HttpResponse::Ok().body(format!("Hello {name}!"))

//  }


//  async fn health_check()-> impl Responder{
//     HttpResponse::Ok().body("ok")
//  }


//  #[tokio::main]

//  async fn main ()-> std:: io:: Result<()>{
//     HttpServer::new(||{
//         App::new()
//         .route("/hello/{name}", web::get().to(greet))
//         .route("/hello", web :: get().to(greet))
//         .route("/health_check", web::get().to(health_check))

//     })
//     .bind("127.0.0.1:8080")?
//     .run()
//     .await
//  }


pub struct TextIndexer<'a> {
    text: &'a str,
    words: Vec<&'a str>,
}

impl<'a> TextIndexer<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, words: Vec::new() }
    }

    pub fn index_words(&mut self) {
        self.words = self.text.split_whitespace().collect();
    }

    pub fn words(&self) -> &Vec<&'a str> {
        &self.words
    }
}

fn main() {
    let text = "Rust makes lifetimes explicit and safe".to_string();
    let mut indexer = TextIndexer::new(&text); // borrow text

    indexer.index_words();

    for word in indexer.words() {
        println!("{}", word);
    }
}