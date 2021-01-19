// On separate lines on purpose so I can add comments later or sooner
use actix_web::get;
use actix_web::HttpResponse;
use actix_web::Responder;

// Request handlers are all async functions.
//
// "get" here is an attribute-like macro:
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

fn main() {
    println!("Starting...");
}
