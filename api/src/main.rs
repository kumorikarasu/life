//#![feature(trace_macros)]
//trace_macros!(true);

#[allow(unused)]

use actix_web::{App, HttpServer, web};
mod life;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    // No idea how to get around this for now
    let s = life::service();

    HttpServer::new(move || {
        App::new()
            .service(web::scope("/life")
                .app_data(web::Data::new(s.clone()))
                .configure(life::configure))
    })
    .bind(("0.0.0.0", 8000))?
    .run()
    .await
}
