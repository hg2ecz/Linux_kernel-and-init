use actix_multipart::Multipart;
use actix_web::{web, App, HttpResponse, HttpServer, Result};
use futures_util::StreamExt;
use std::io::Write;
use tera::Tera;

mod models;
mod schema;

mod set_ethernet;
mod sysdiag;

const ETH_IF: &str = "eth0";
const DIAG_PORT: u16 = 7878;

async fn index(tmpl: web::Data<Tera>) -> Result<HttpResponse> {
    let rendered = tmpl.render("index.html", &tera::Context::new()).unwrap();
    Ok(HttpResponse::Ok().content_type("text/html").body(rendered))
}

async fn upload(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    while let Some(item) = payload.next().await {
        let mut field = item?;

        let filename = field
            .content_disposition()
            .get_filename()
            .unwrap()
            .to_string();
        let filepath = format!("./uploads/{}", filename);

        let mut f = web::block(|| std::fs::File::create(filepath)).await??;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f = web::block(move || f.write_all(&data).map(|_| f)).await??;
        }
    }
    Ok(HttpResponse::Ok().body("File uploaded successfully"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\nHello WebTest from Rust! Diag TCP port: {DIAG_PORT}");
    let _ = set_ethernet::set_interface_up(ETH_IF); // ifup
    sysdiag::Diag::new(DIAG_PORT); // TCP diag port

    let tera = Tera::new("/templates/**/*").unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(web::resource("/").route(web::get().to(index)))
            .service(web::resource("/upload").route(web::post().to(upload)))
    })
    .bind(":::8080")?
    .run()
    .await
}
