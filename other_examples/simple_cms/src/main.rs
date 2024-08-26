use actix_web::{web, App, HttpServer};

mod set_ethernet;
mod sysdiag;

const ETH_IF: &str = "eth0";
const DIAG_PORT: u16 = 7878;

mod file_local;
mod file_sql;
mod file_tcp;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("\nHello WebTest from Rust! Diag TCP port: {DIAG_PORT}");
    let _ = set_ethernet::set_interface_up(ETH_IF); // ifup
    sysdiag::Diag::new(DIAG_PORT); // TCP diag port

    let fsql = file_sql::sql_init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(fsql.clone()))
            .route("/sqlfile/{filename}", web::get().to(file_sql::get_file))
            .route("/localfile/{filename}", web::get().to(file_local::get_file))
            .route("/tcpfile/{filename}", web::get().to(file_tcp::get_file))
    })
    .bind(":::8080")?
    .run()
    .await
}
