use std::fs::File;
use std::io::{Read, Write};
use std::net::{Shutdown, TcpListener, TcpStream};

static FILE_BASE: &str = "/tmp/";

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    let mut content = Vec::new();

    let mut flenbytes = [0u8; 1];
    stream.read(&mut flenbytes)?;
    let mut filename_vec = vec![0u8; flenbytes[0] as usize];
    stream.read(&mut filename_vec)?;
    let filename = FILE_BASE.to_string() + &String::from_utf8_lossy(&filename_vec);
    if filename.contains("..") {
        return Ok(());
    }
    if let Ok(mut f) = File::open(&*filename) {
        f.read_to_end(&mut content).expect("buffer overflow");
    }
    let mut flen: [u8; 4] = u32::to_be_bytes(content.len() as u32);
    stream.write(&mut flen)?;
    stream.write(&mut content)?;
    stream.shutdown(Shutdown::Both)?;

    Ok(())
}

fn main() {
    let listener = TcpListener::bind(":::8777").expect("TCP bind error. Stop.");
    for stream_in in listener.incoming() {
        if let Ok(stream) = stream_in {
            let _ = handle_client(stream);
        }
    }
}
