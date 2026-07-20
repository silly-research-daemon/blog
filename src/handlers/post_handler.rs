use actix_web::{web, get, Responder, HttpResponse};
use pulldown_cmark::{Options, Parser, html};
use std::fs;
use std::io::Error;

use super::home_handler::Frontmatter;

#[get("/posts/{post_name}")]
pub async fn post(
    tmpl: web::Data<tera::Tera>,
    post_name: web::Path<String>,
) -> impl Responder {
    let post_name = post_name.into_inner();

    if post_name.contains('/') || post_name.contains('\\') || post_name.contains("..") {
        return HttpResponse::NotFound()
            .content_type("text/html")
            .body("<p>Could not find post - sorry!</p>");
    }

    let mut context = tera::Context::new();

    let markdown_input = match extract_markdown(&post_name) {
        Ok(s) => s,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post - sorry!</p>");
        }
    };

    let frontmatter = match extract_frontmatter(&post_name) {
        Ok(fm) => fm,
        Err(e) => {
            println!("{:?}", e);
            return HttpResponse::NotFound()
                .content_type("text/html")
                .body("<p>Could not find post - sorry!</p>");
        }
    };

    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_TASKLISTS);

    let parser = Parser::new_ext(&markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    context.insert("post", &html_output);
    context.insert("meta_data", &frontmatter);

    match tmpl.render("post.html", &context) {
        Ok(s) => HttpResponse::Ok().content_type("text/html").body(s),
        Err(e) => {
            println!("{:?}", e);
            HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("<p>Something went wrong!</p>")
        }
    }
}

fn extract_markdown(post_name: &str) -> Result<String, Error> {
    fs::read_to_string(format!("./posts/{}/post.md", post_name))
}

fn extract_frontmatter(post_name: &str) -> Result<Frontmatter, Error> {
    let content = fs::read_to_string(format!("./posts/{}/post_frontmatter.toml", post_name))?;
    toml::from_str(&content).map_err(|e| Error::new(std::io::ErrorKind::InvalidData, e))
}
