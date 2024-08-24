use mysql::prelude::*;
use mysql::*;

mod set_ethernet;
mod sysdiag;

const ETH_IF: &str = "eth0";
const DIAG_PORT: u16 = 7878;


const DATABASE_URL: &str = "mysql://test:testpwd@[fd73::d250:99ff:fe59:e012]/test";
// const DATABASE_URL: &str = "mysql://test:testpwd@192.168.233.206/test";

fn get_mysql_pool() -> Pool {
    Pool::new(DATABASE_URL).expect("Failed to create MySQL connection pool")
}

#[derive(Debug, serde::Serialize)]
struct FileEntry {
    id: i32,
    filename: String,
    content: Vec<u8>,
    content_type: String,
}

fn get_file_from_db(pool: &Pool, filename: &str) -> Result<Option<FileEntry>> {
    let mut conn = pool.get_conn()?;
    let result = conn.exec_first(
        "SELECT id, filename, content, content_type FROM files WHERE filename = :filename",
        params! { "filename" => filename },
    )?;

    if let Some((id, filename, content, content_type)) = result {
        Ok(Some(FileEntry {
            id,
            filename,
            content,
            content_type,
        }))
    } else {
        Ok(None)
    }
}

use actix_web::http::header::ContentType;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

async fn serve_file(pool: web::Data<Pool>, filename: web::Path<String>) -> impl Responder {
    match get_file_from_db(&pool, &filename) {
        Ok(Some(file_entry)) => HttpResponse::Ok()
            .insert_header(ContentType(file_entry.content_type.parse().unwrap()))
            .body(file_entry.content),
        Ok(None) => HttpResponse::NotFound().body("File not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error loading file"),
    }
}

fn insert_file(pool: &Pool, filename: &str, content: Vec<u8>, content_type: &str) -> Result<()> {
    let mut conn = pool.get_conn()?;
    conn.exec_drop(
        "INSERT INTO files (filename, content, content_type) VALUES (:filename, :content, :content_type)",
        params! {
            "filename" => filename,
            "content" => content,
            "content_type" => content_type,
        },
    )?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\nHello WebTest from Rust! Diag TCP port: {DIAG_PORT}");
    let _ = set_ethernet::set_interface_up(ETH_IF); // ifup
    sysdiag::Diag::new(DIAG_PORT); // TCP diag port

    let pool = get_mysql_pool();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/files/{filename}", web::get().to(serve_file))
    })
    .bind(":::8080")?
    .run()
    .await
}
