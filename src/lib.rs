use actix_files::Files;
use actix_web::{dev::Server, middleware, web, App, HttpResponse, HttpServer};
use std::net::TcpListener;
use tera::Tera;

pub mod handlers;

#[macro_use]
extern crate lazy_static;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = Tera::new();
        if let Err(e) = tera.load_from_glob("templates/**/*.html") {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
        tera.autoescape_on(vec![".html", ".sql"]);
        tera
    };
}

pub fn start_blog(listener: TcpListener) -> Result<Server, std::io::Error> {
    let srv = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(TEMPLATES.clone()))
            .wrap(middleware::Logger::default())
            .service(Files::new("/static", "static/").use_last_modified(true))
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(handlers::index)
            .service(handlers::post)
    })
        .listen(listener)?
        .run();

    Ok(srv)
}
