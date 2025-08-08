use actix_web::{get, post, web, App, HttpServer, Responder, HttpResponse};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain; charset=utf-8")
        .body(" {Ω, ∅} σ-álgebra trivial como dice la chaviza")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().content_type("text/plain; charset=utf-8")
        .body(" {Ω, ∅} σ-álgebra trivial como dice la chaviza")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
            .service(echo)
            .route("/trivia", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 4444))?
    .run()
    .await
}