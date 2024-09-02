use std::collections::VecDeque;
use std::ffi::CString;
use std::fs::{self, read_to_string, DirEntry};
use std::io::{self, BufRead, BufReader, Write};
use std::net::{IpAddr, Shutdown, TcpListener, TcpStream};
use std::os::unix::fs::{FileTypeExt, MetadataExt, PermissionsExt};
use std::path::{Path, PathBuf};
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
        thread::spawn(move || loop {
            handle_stdio();
        });
        thread::spawn(move || {
            let listener = TcpListener::bind(format!(":::{port}")).unwrap();

            for tcpstream in listener.incoming() {
                let _ = handle_tcp_client(tcpstream.unwrap());
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

static VALID_COMMANDS: [&str; 9] = [
    "meminfo",
    "loadavg",
    "proc",
    "mounts",
    "listfiles",
    "version",
    "reboot",
    "pwroff",
    "quit",
];

fn handle_stdio() {
    let validcmd = VALID_COMMANDS.join(" ");
    println!("{validcmd}");
    let mut buf = String::new();

    loop {
        buf.clear();
        io::stdin().read_line(&mut buf).expect("Failed to read line");
        if buf.len() == 0 {
            println!("Zero byte from stdin. Exit from the stdin handler.");
            std::process::exit(0);
        }
        let cmd = buf.trim();
        match cmd {
            "proc" => {
                println!("{}", listproc_only_numeric());
            }
            "reboot" => {
                println!("System reboot ...");
                let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_RESTART) };
            }
            "pwroff" => {
                if Path::new("/.dockerenv").exists() {
                    println!("Docker \"poweroff\" ...");
                    std::process::exit(0);
                } else {
                    println!("System poweroff ...");
                    let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF) };
                }
            }
            "listfiles" => {
                // let _ = listfiles(&mut stream);
                println!("Implemented only on TCP/IP.");
            }
            "quit" => {
                break;
            }
            _ => {
                if VALID_COMMANDS.contains(&cmd) {
                    if let Ok(buf) = fs::read_to_string(format!("/proc/{cmd}")) {
                        println!("{buf}");
                    }
                } else {
                    println!("Unknown command: \"{cmd}\"\nValid commands: {validcmd}");
                }
            }
        }
    }
}

fn handle_tcp_client(stream: TcpStream) -> Result<(), std::io::Error> {
    let mut reader = BufReader::new(stream.try_clone()?);
    let mut stream = stream; // Now we have a separate mutable stream for writing
    let validcmd = VALID_COMMANDS.join(" ");
    writeln!(stream, "{validcmd}")?;

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
                        if Path::new("/.dockerenv").exists() {
                            writeln!(stream, "Docker \"poweroff\" ...")?;
                            stream.shutdown(Shutdown::Both)?;
                            std::process::exit(0);
                        } else {
                            writeln!(stream, "System poweroff ...")?;
                            stream.shutdown(Shutdown::Both)?;
                            let _ = unsafe { libc::reboot(libc::LINUX_REBOOT_CMD_POWER_OFF) };
                        }
                    }
                    "listfiles" => {
                        let _ = listfiles(&mut stream);
                    }
                    "quit" => {
                        stream.shutdown(Shutdown::Both)?;
                        break;
                    }
                    _ => {
                        if VALID_COMMANDS.contains(&cmd) {
                            if let Ok(buf) = fs::read_to_string(format!("/proc/{cmd}")) {
                                writeln!(stream, "{buf}")?;
                            }
                        } else {
                            writeln!(stream, "Unknown command: \"{cmd}\"\nValid commands: {validcmd}")?;
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

fn listfiles(stream: &mut TcpStream) -> io::Result<()> {
    let mut dirs_to_visit: VecDeque<PathBuf> = VecDeque::new();
    dirs_to_visit.push_back("/".into());

    let _ = writeln!(stream, "   TYPE PERM UID GID      SIZE      FILENAME");
    while let Some(current_dir) = dirs_to_visit.pop_front() {
        if current_dir == Path::new("/proc") || current_dir == Path::new("/sys") {
            continue;
        }
        let _ = writeln!(stream, "{}:", current_dir.display());
        let entries = fs::read_dir(&current_dir)?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            let metadata = fs::symlink_metadata(entry.path())?;
            if path.is_dir() && !metadata.file_type().is_symlink() {
                dirs_to_visit.push_back(path.clone());
            }
            display_metadata(&entry, stream)?;
        }
    }
    Ok(())
}

fn display_metadata(entry: &DirEntry, stream: &mut TcpStream) -> io::Result<()> {
    let metadata = entry.metadata()?;
    let ftype = &metadata.file_type();
    let file_type = if metadata.is_dir() {
        "dir "
    } else if metadata.is_file() {
        "file"
    } else if metadata.is_symlink() {
        "link"
    } else if ftype.is_char_device() {
        "cdev"
    } else if ftype.is_block_device() {
        "bdev"
    } else if ftype.is_fifo() {
        "fifo"
    } else if ftype.is_socket() {
        "sock"
    } else {
        "?   "
    };
    let permissions = metadata.permissions().mode() & 0o777;
    let user_id = metadata.uid();
    let group_id = metadata.gid();
    let fsize = metadata.len();
    let file_size = match fsize {
        ..1024 => format!("{fsize}"),
        1024..1048576 => format!("{:.1} kB", fsize as f64 / 1024.),
        1048576.. => format!("{:.1} MB", fsize as f64 / 1024. / 1024.),
    };
    let mut filename = entry.path().display().to_string();
    if file_type == "link" {
        if let Ok(target_path) = fs::read_link(entry.path()) {
            filename = format!("{filename} -> {}", target_path.display());
        }
    }
    let _ = writeln!(
        stream,
        "   {file_type} {permissions:o} {user_id:4} {group_id:4} {file_size:8}  {filename}"
    );
    Ok(())
}
