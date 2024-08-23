use std::ffi::CString;
use std::fs::{self, read_to_string};
use std::io::{BufRead, BufReader, Write};
use std::net::{IpAddr, Shutdown, TcpListener, TcpStream};
use std::path::Path;
use std::str::FromStr;
use std::thread;
use std::time::Duration;

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

        // 1s delay
        let sleep_time = Duration::from_millis(1500);
        thread::sleep(sleep_time);

        if let Ok(buf) = read_to_string("/proc/net/dev") {
            println!("/proc/net/dev:\n{buf}");
        }
        if let Ok(buf) = read_to_string("/proc/net/if_inet6") {
            for row in buf.split("\n") {
                if let Some(s) = row.split(' ').next() {
                    let ipv6_str = s.chars().enumerate().fold(String::new(), |mut acc, (i, c)| {
                        if i > 0 && i % 4 == 0 {
                            acc.push(':');
                        }
                        acc.push(c);
                        acc
                    });
                    if let Ok(ipv6) = IpAddr::from_str(&ipv6_str) {
                        println!("IPv6 addr: {ipv6}");
                    }
                }
            }
        }
        println!();
        Diag {}
    }
}

fn handle_client(stream: TcpStream) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut stream = stream; // Now we have a separate mutable stream for writing
    let valid_commands = ["meminfo", "loadavg", "proc", "mounts", "reboot", "pwroff", "quit"];
    writeln!(stream, "{}", valid_commands.join(" "))?;

    loop {
        let mut buf = String::new();
        match reader.read_line(&mut buf) {
            Ok(0) => break,
            Ok(_) => {
                let cmd = buf.trim();
                match cmd {
                    "proc" => {
                        writeln!(stream, "{}", listproc_only_numeric())?;
                    }
                    "reboot" => {
                        writeln!(stream, "System reboot ...")?;
                        stream.shutdown(Shutdown::Both)?;
                        let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_RESTART) };
                    }
                    "pwroff" => {
                        writeln!(stream, "System poweroff ...")?;
                        stream.shutdown(Shutdown::Both)?;
                        let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF) };
                    }
                    "quit" => {
                        stream.shutdown(Shutdown::Both)?;
                        break;
                    }
                    _ => {
                        if valid_commands.contains(&cmd) {
                            if let Ok(buf) = fs::read_to_string(format!("/proc/{cmd}")) {
                                writeln!(stream, "{buf}")?;
                            }
                        } else {
                            writeln!(stream, "Unknown command: {cmd}")?;
                        }
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

fn proc_statusgen(s: &str) -> String {
    let mut pinf = String::new();
    if let Ok(content_str) = read_to_string(format!("{s}/status")) {
        for c in content_str.split("\n") {
            let mut spl = c.split(":");
            let Some(key) = spl.next() else { break };
            let Some(val) = spl.next() else { break };
            let val = val.trim();
            match key {
                "Name" => {
                    pinf += &format!("Name: {val:32}");
                }
                "VmRSS" => {
                    pinf += &format!(" VmRSS: {val:8}");
                }
                "Kthread" => {
                    if val == "1" {
                        pinf += " KERNEL ";
                    }
                }
                "Threads" => {
                    pinf += &format!(" Threads: {val:8}");
                }
                "VmData" => {
                    pinf += &format!(" VmData: {val:8}");
                }
                "VmStk" => {
                    pinf += &format!(" VmStk: {val:8}");
                }
                "VmExe" => {
                    pinf += &format!(" VmExe: {val:8}");
                }
                "VmLib" => {
                    pinf += &format!(" VmLib: {val:8}");
                }
                "VmPTE" => {
                    pinf += &format!(" VmPTE: {val:8}");
                }
                "VmSwap" => {
                    pinf += &format!(" VmSwap: {val:8}");
                }
                _ => (),
            }
        }
    }
    format!("{s:<10} {pinf}\n")
}

fn listproc_only_numeric() -> String {
    let mut txt = String::new();
    let path = Path::new("/proc");
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(file_name) = path.file_name() {
                if let Some(file_name_str) = file_name.to_str() {
                    if file_name_str.chars().next().map_or(false, |c| c.is_numeric()) {
                        let s = path.display().to_string();
                        txt += &proc_statusgen(&s);
                    }
                }
            }
        }
    }
    txt
}
