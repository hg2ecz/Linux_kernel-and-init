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

            for tcpstream in listener.incoming() {
                let _ = handle_client(tcpstream.unwrap());
            }
        });
        Diag {}
    }
}

fn handle_client(stream: TcpStream) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut stream = stream; // Now we have a separate mutable stream for writing
    writeln!(stream, "meminfo loadavg listproc reboot pwroff quit\n")?;

    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let cmd = buf.trim();
                match cmd {
                    "meminfo" => {
                        if let Ok(buf) = fs::read_to_string("/proc/meminfo") {
                            writeln!(stream, "{buf}")?;
                        }
                    }
                    "loadavg" => {
                        if let Ok(buf) = fs::read_to_string("/proc/loadavg") {
                            writeln!(stream, "{buf}")?;
                        }
                    }
                    "listproc" => {
                        writeln!(stream, "{}", listproc_only_numeric())?;
                    }
                    "reboot" => {
                        writeln!(stream, "System reboot ...\n")?;
                        stream.shutdown(Shutdown::Both)?;
                        let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_RESTART) };
                    }
                    "pwroff" => {
                        writeln!(stream, "System poweroff ...\n")?;
                        stream.shutdown(Shutdown::Both)?;
                        let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF) };
                    }
                    "quit" => {
                        stream.shutdown(Shutdown::Both)?;
                        break;
                    }
                    _ => {
                        writeln!(stream, "Unknown command: {cmd}\n")?;
                    }
                }
            }
            Err(e) => {
                println!("Failed to read from the client: {}", e);
                break;
            }
        }
    }
    Ok(())
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
