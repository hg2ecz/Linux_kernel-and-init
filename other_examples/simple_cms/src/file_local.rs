use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};
use std::fs::File;
use std::io::Read;

#[derive(Debug, serde::Serialize)]
struct FileEntry {
    filename: String,
    content: Vec<u8>,
    content_type: String,
}

fn get_file_from_local(filename: &str) -> Option<FileEntry> {
    let realfile = format!("/webdata/{filename}");
    if let Ok(mut f) = File::open(&realfile) {
        let mut content = Vec::new();
        f.read_to_end(&mut content).expect("buffer overflow");

        let extens: Vec<&str> = filename.split(".").collect();
        let content_type = match extens[extens.len() - 1] {
            "html" => "text/html; charset=UTF-8",
            "jpg" => "image/jpeg",
            "png" => "image/png",
            "gif" => "image/gif",
            _ => "text/plain",
        };

        Some(FileEntry {
            filename: filename.to_string(),
            content,
            content_type: content_type.to_string(),
        })
    } else {
        None
    }
}

//pub async fn get_file(pool: web::Data<Pool>, filename: web::Path<String>) -> impl Responder {
pub async fn get_file(filename: web::Path<String>) -> impl Responder {
    match get_file_from_local(&filename) {
        Some(file_entry) => HttpResponse::Ok()
            .insert_header(ContentType(file_entry.content_type.parse().unwrap()))
            .body(file_entry.content),
        None => HttpResponse::NotFound().body("File not found"),
    }
}

/*
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
*/
