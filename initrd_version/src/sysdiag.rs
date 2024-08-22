use std::ffi::CString;
use std::fs::{self, read_to_string};
use std::io::{BufRead, BufReader, Write};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::path::Path;
use std::thread;

pub struct Diag {}

impl Diag {
    pub fn new(port: u16) -> Self {
        let msrc = CString::new("proc").unwrap();
        let mdst = CString::new("proc").unwrap();
        let mtype = CString::new("proc").unwrap();
        let mflags = 0;
        unsafe {
            libc::mount(msrc.as_ptr(), mdst.as_ptr(), mtype.as_ptr(), mflags, std::ptr::null());
        }
        thread::spawn(move || {
            let listener = TcpListener::bind(format!(":::{port}")).unwrap();

            for stream in listener.incoming() {
                handle_client(stream.unwrap());
            }
        });
        Diag {}
    }
}

fn handle_client(mut stream: TcpStream) {
    stream.write_all(b"meminfo loadavg listproc reboot pwroff\n").unwrap();
    let mut reader = BufReader::new(&mut stream);
    let mut buffer = String::new();
    match reader.read_line(&mut buffer) {
        Ok(_) => {
            let cmd = buffer.trim();
            match cmd {
                "meminfo" => {
                    if let Ok(buf) = fs::read_to_string("/proc/meminfo") {
                        let _ = stream.write_all(buf.as_bytes());
                    }
                }
                "loadavg" => {
                    if let Ok(buf) = fs::read_to_string("/proc/loadavg") {
                        let _ = stream.write_all(buf.as_bytes());
                    }
                }
                "listproc" => {
                    let _ = stream.write_all(listproc_only_numeric().as_bytes());
                }
                "reboot" => {
                    let _ = stream.write_all(b"System reboot ...\n");
                    let _ = stream.shutdown(Shutdown::Both);
                    let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_RESTART) };
                }
                "pwroff" => {
                    let _ = stream.write_all(b"System poweroff ...\n");
                    let _ = stream.shutdown(Shutdown::Both);
                    let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF) };
                }
                _ => {
                    let _ = stream.write_all("Unknown command: {cmd}\n".as_bytes());
                }
            }
        }
        Err(e) => {
            println!("Failed to read from the stream: {}", e);
        }
    }
}

fn listproc_only_numeric() -> String {
    let mut txt = String::new();
    let path = Path::new("/proc");
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if file_name_str.chars().next().map_or(false, |c| c.is_numeric()) {
                            let s = path.display().to_string();
                            if let Ok(content_str) = read_to_string(format!("{s}/status")) {
                                let mut content = content_str.split("\n");
                                if let Some(name) = content.next() {
                                    txt += &format!("{s:<10} {name}\n");
                                } else {
                                    txt += &format!("{s}");
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    txt
}

//fn read_diag(mut stream: TcpStream, diagfile: &str) {
pub fn read_diag(diagfile: &str) -> Result<String, std::io::Error> {
    read_to_string(diagfile)
}
