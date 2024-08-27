use actix_web::http::header::ContentType;
use actix_web::{web, HttpResponse, Responder};

use std::io::{self, Read, Write};
use std::net::TcpStream;

const TCPSERVER: &str = "[fd73::d250:99ff:fe59:e012]:8777";

#[derive(Debug, serde::Serialize)]
struct FileEntry {
    filename: String,
    content: Vec<u8>,
    content_type: String,
}

fn get_file_from_tcp(filename: &str) -> Result<Option<FileEntry>, io::Error> {
    let mut stream = TcpStream::connect(TCPSERVER)?;
    let fnlen = [filename.len() as u8; 1];
    stream.write(&fnlen)?;
    stream.write(filename.as_bytes())?;
    let mut flenbytes = [0u8; 4];
    stream.read(&mut flenbytes)?;
    let flen = u32::from_be_bytes(flenbytes);
    let mut content = vec![0u8; flen as usize];
    stream.read(&mut content)?;

    let extens: Vec<&str> = filename.split(".").collect();
    let content_type = match extens[extens.len() - 1] {
        "html" => "text/html; charset=UTF-8",
        "jpg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        _ => "text/plain",
    };

    Ok(Some(FileEntry {
        filename: filename.to_string(),
        content,
        content_type: content_type.to_string(),
    }))
}

pub async fn get_file(filename: web::Path<String>) -> impl Responder {
    match get_file_from_tcp(&filename) {
        Ok(Some(file_entry)) => HttpResponse::Ok()
            .insert_header(ContentType(file_entry.content_type.parse().unwrap()))
            .body(file_entry.content),
        Ok(None) => HttpResponse::NotFound().body("File not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error loading file"),
    }
}
