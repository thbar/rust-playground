// On separate lines on purpose so I can add comments later or sooner
use actix_web::get;
use actix_web::post;
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

fn main() {
    println!("Starting...");
}
