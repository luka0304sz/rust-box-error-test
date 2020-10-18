extern crate env_logger;

use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use chrono::DateTime;

async fn version(_req: HttpRequest) -> HttpResponse {
    HttpResponse::Ok().json("0.1.0")
}

async fn error(_req: HttpRequest) -> HttpResponse {
    let response = move || -> Result<(), Box<dyn std::error::Error>> {
        // let _dt = DateTime::parse_from_rfc2822("Friiii, 14 Jul 2017 02:40:00 +0000")?;
        let _n: u32 = "m".parse()?;
        Ok(())
    };

    match response() {
        Ok(_) => HttpResponse::Ok().json("ok"),
        Err(err) => {
            println!("{:?}", err);
            let message = match err {
                // chrono::format::ParseError => "error converting time", //ParseError(Invalid)
                // std::num::ParseIntError => "error converting to number", //ParseIntError { kind: InvalidDigit }
                _ => "unknown error",
            };
            HttpResponse::BadRequest().json(message)
        }
    }
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");
    // env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(version))
            .route("/error", web::get().to(error))
    })
    .bind("0.0.0.0:4001")?
    .run()
    .await
}
