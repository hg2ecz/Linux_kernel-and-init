use chrono::prelude::*;
use std::{thread, time};
mod set_ethernet;
mod sysdiag;

const ETH_IF: &str = "eth0";
const DIAG_PORT: u16 = 7878;

fn main() {
    println!("\nHello, world from Rust! Diag TCP port: {DIAG_PORT}");
    let _ = set_ethernet::set_interface_up(ETH_IF); // ifup
    sysdiag::Diag::new(DIAG_PORT); // TCP diag port

    let sleep_time = time::Duration::from_millis(10000);
    loop {
        thread::sleep(sleep_time);
        let local: DateTime<Local> = Local::now();
        println!("{:?} Hello, world from Rust again!", local);
    }
}
