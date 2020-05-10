use actix_web::web::Path;
use actix_web::{get, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Name {
  name: String,
  count: i32,
}

#[get("")]
pub async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello world!")
}

#[get("/again")]
pub async fn get_again() -> impl Responder {
  HttpResponse::Ok().body("Hello world again!")
}

#[get("/{name}")]
pub async fn get_name(path: Path<(String,)>) -> HttpResponse {
  // get path param from String
  HttpResponse::Ok().body(format!("Hey {} there!", path.0))
}

#[get("/me/{name}/{count}")]
pub async fn get_name_and_count(path: Path<Name>) -> HttpResponse {
  // get path param from struct
  HttpResponse::Ok().body(format!("Hey {} there! {}", path.name, path.count))
}

#[get("/hello_more")]
pub async fn hello_more() -> impl Responder {
  HttpResponse::Ok().body("Hey there!")
}
