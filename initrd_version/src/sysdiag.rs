use std::ffi::CString;
use std::fs::{self, read_to_string};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

pub struct Diag {}

impl Diag {
    pub fn new() -> Self {
        let msrc = CString::new("proc").unwrap();
        let mdst = CString::new("proc").unwrap();
        let mtype = CString::new("proc").unwrap();
        let mflags = 0;
        unsafe {
            libc::mount(
                msrc.as_ptr(),
                mdst.as_ptr(),
                mtype.as_ptr(),
                mflags,
                std::ptr::null(),
            );
        }
        thread::spawn(move || {
            let listener = TcpListener::bind(":::7878").unwrap();

            for stream in listener.incoming() {
                handle_client(stream.unwrap());
            }
        });
        Diag {}
    }
}

fn handle_client(mut stream: TcpStream) {
    stream.write_all("meminfo reboot\n".as_bytes()).unwrap();
    //stream.flush().unwrap();
    let mut reader = BufReader::new(&mut stream);
    let mut buffer = String::new();
    match reader.read_line(&mut buffer) {
        Ok(_) => {
            let cmd = buffer.trim();
            match cmd {
                "meminfo" => {
                    if let Ok(buf) = fs::read_to_string("/proc/meminfo") {
                        stream.write_all(buf.as_bytes()).unwrap();
                    }
                }
                "reboot" => {
                    stream.write_all("System reboot ...\n".as_bytes()).unwrap();
                    let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_RESTART) };
                }
                _ => stream
                    .write_all("Unknown command: {cmd}\n".as_bytes())
                    .unwrap(),
            }
        }
        Err(e) => {
            println!("Failed to read from the stream: {}", e);
        }
    }
}

//fn read_diag(mut stream: TcpStream, diagfile: &str) {
pub fn read_diag(diagfile: &str) -> Result<String, std::io::Error> {
    read_to_string(diagfile)
}
