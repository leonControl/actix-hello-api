use actix_web::{middleware, web, App, HttpServer};
use env_logger;
use std::{env, io};

mod handlers;

// this function could be located in different module
fn config(cfg: &mut web::ServiceConfig) {
  cfg
    .service(handlers::hello)
    .service(handlers::get_again)
    .service(handlers::get_name)
    .service(handlers::get_name_and_count);
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
  env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
  env_logger::init();
  HttpServer::new(|| {
    App::new()
      // enable logger - always register actix-web Logger middleware last
      .wrap(middleware::Logger::default())
      .wrap(middleware::Logger::new("%a %{User-Agent}i"))
      // register HTTP requests handlers
      .service(web::scope("/hello").configure(config))
      .service(handlers::hello_more)
  })
  .bind("127.0.0.1:8088")?
  .run()
  .await
}
