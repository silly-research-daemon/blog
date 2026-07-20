use std::net::TcpListener;

use jwcbmat_blog::start_blog;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    unsafe {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    env_logger::init();

    let listener = TcpListener::bind("127.0.0.1:8080")?;
    start_blog(listener)?.await?;
    Ok(())
}
