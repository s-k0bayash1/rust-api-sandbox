use actix_web::{HttpResponse, Responder};

pub fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}