// On separate lines on purpose so I can add comments later or sooner
use actix_web::get;
use actix_web::post;
use actix_web::web;
use actix_web::App;
use actix_web::HttpResponse;
use actix_web::HttpServer;
use actix_web::Responder;

// Request handlers are all async functions.
//
// "get" here is an attribute-like macro (`proc_macro_attribute`):
// https://doc.rust-lang.org/book/ch19-06-macros.html?highlight=attribute#attribute-like-macros
// It is a way for Actix to reduce boilerplate code in Actix Web applications.
// The code is here: https://github.com/actix/actix-web/blob/master/actix-web-codegen/src/lib.rs
//
// The value returned by the method must be something for which the actix_web::Responder trait is implemented
// https://docs.rs/actix-web/3.3.2/actix_web/trait.Responder.html
// See https://actix.rs/docs/handlers/ for more information
//
#[get("/")]
async fn hello() -> impl Responder {
    // Ok() here is a function - see:
    // https://docs.rs/actix-web/3.3.2/actix_web/struct.HttpResponse.html
    // It returns a HttpResponseBuilder:
    // https://docs.rs/actix-web/3.3.2/actix_web/dev/struct.HttpResponseBuilder.html
    HttpResponse::Ok().body("Hello world!")
}

// If we don't use the attribute-like macro, there will be a bit more leg-work not here,
// but in the route registering code (see main).
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

// Parameters can be extracted from a request thanks to the FromRequest trait:
// https://docs.rs/actix-web/3.3.2/actix_web/trait.FromRequest.html
#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    // Here we just echo back the content, sent by POST
    //
    // ⚠️ Make sure **not** to add a trailing ";" or you'll get an error message
    // (the trait `Responder` is not implemented for `()`)
    HttpResponse::Ok().body(req_body)
}

// Here we see another use of `proc_macro_attribute`, implemented here:
// https://github.com/actix/actix-web/blob/0c8196f8b07022e56fb4f967c26426cd3076ffdf/actix-web-codegen/src/lib.rs#L169
//
// It is syntactic sugar on top of `actix_web::rt::System::new`, which is an alias to:
// https://docs.rs/actix-rt/1.1.1/actix_rt/ System struct.
//
// actix-rt (https://crates.io/crates/actix-rt) is a:
// "Tokio-based single-thread async runtime for the Actix ecosystem"
//
// Tokio is an [asynchronous runtime for Rust](https://tokio.rs).
#[actix_web::main]
// TODO: detail the full chain (bind/run/await) and most importantly why we need this return type
async fn main() -> std::io::Result<()> {
    // The || { } construct is just a zero-parameter closure. It allows to
    // defer the evaluation to later (at the appropriate time).
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/manual_hello", web::get().to(manual_hello))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
