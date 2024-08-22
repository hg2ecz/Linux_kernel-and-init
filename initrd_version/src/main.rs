use chrono::prelude::*;
use std::{thread, time};
mod set_ethernet;
mod sysdiag;

fn main() {
    let diag_port = 7878;
    println!("\nHello, world from Rust! Diag TCP port: {diag_port}");
    let _ = set_ethernet::set_interface_up("eth0"); // ifup
    sysdiag::Diag::new(diag_port); // TCP diag port

    // 1s delay
    let sleep_time = time::Duration::from_millis(2000);
    thread::sleep(sleep_time);

    let files = ["/proc/net/dev", "/proc/net/if_inet6"];
    for file in files {
        if let Ok(buf) = sysdiag::read_diag(file) {
            println!("{file}:\n{buf}");
        }
    }

    let sleep_time = time::Duration::from_millis(10000);
    loop {
        thread::sleep(sleep_time);
        let local: DateTime<Local> = Local::now();
        println!("{:?} Hello, world again from Rust!", local);
    }
}
