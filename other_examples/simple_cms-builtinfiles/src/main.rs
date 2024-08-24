use actix_web::{web, App, HttpResponse, HttpServer};
use rust_embed::RustEmbed;
use tera::Tera;

mod set_ethernet;
mod sysdiag;

const ETH_IF: &str = "eth0";
const DIAG_PORT: u16 = 7878;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

async fn index() -> HttpResponse {
    // Load the template from the embedded assets
    let template = Asset::get("index.html").unwrap();
    let content = std::str::from_utf8(template.data.as_ref()).unwrap();

    // Create Tera instance from the embedded template
    let mut tera = Tera::default();
    tera.add_raw_template("index.html", content).unwrap();

    let rendered = tera.render("index.html", &tera::Context::new()).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\nHello WebTest from Rust! Diag TCP port: {DIAG_PORT}");
    let _ = set_ethernet::set_interface_up(ETH_IF); // ifup
    sysdiag::Diag::new(DIAG_PORT); // TCP diag port

    HttpServer::new(|| App::new().service(web::resource("/").route(web::get().to(index))))
        .bind(":::8080")?
        .run()
        .await
}
