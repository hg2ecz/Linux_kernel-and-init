use mysql::prelude::*;
use mysql::*;

use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};

const DATABASE_URL: &str = "mysql://test:testpwd@[fd73::d250:99ff:fe59:e012]/test";
// const DATABASE_URL: &str = "mysql://test:testpwd@192.168.233.206/test";

pub fn sql_init() -> Pool {
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

pub async fn get_file(pool: web::Data<Pool>, filename: web::Path<String>) -> impl Responder {
    match get_file_from_db(&pool, &filename) {
        Ok(Some(file_entry)) => HttpResponse::Ok()
            .insert_header(ContentType(file_entry.content_type.parse().unwrap()))
            .body(file_entry.content),
        Ok(None) => HttpResponse::NotFound().body("File not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error loading file"),
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
