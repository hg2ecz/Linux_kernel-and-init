use actix_web::{HttpResponse, Responder};

pub async fn get_greetings() -> impl Responder {
    HttpResponse::Ok().content_type("text/html").body(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Main Page</title>
        </head>
        <body>
            <h1>Welcome to the Main Page</h1>
            Options:
            <ul>
                <li><a href="/localfile/hello.html">Hello from localfile</a></li>
                <li><a href="/sqlfile/hello.html">Hello from SQL</a></li>
                <li><a href="/tcpfile/hello.html">Hello from TCP</a> - from a custom TCP fileserver, if the daemon is running.</li>
        </body>
        </html>
        "#,
    )
}
