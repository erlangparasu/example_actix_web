use actix_files::NamedFile;
use actix_rt::System;
use actix_web::{App, Error, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;
use dotenv::dotenv;
use futures::stream::StreamExt;
use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use sqlx::postgres::PgPool;
use sqlx::types::Uuid;
use sqlx::Row;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;

#[derive(Debug, sqlx::FromRow)]
struct User {
    id: Uuid,
    name: String,
    email: String,
}

async fn index(pool: web::Data<PgPool>) -> impl Responder {
    let mut conn = pool.acquire().await.unwrap();

    // Perform a database query
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
        .bind(Uuid::new_v4())
        .fetch_one(&mut conn)
        .await
        .unwrap();

    HttpResponse::Ok().body(format!("Hello, {} ({})!", user.name, user.email))
}

async fn upload(mut payload: web::Payload) -> Result<HttpResponse, Error> {
    // Generate a unique filename for the uploaded file
    let filename = Uuid::new_v4().to_string();

    // Create a file on the server to store the uploaded data
    let mut file = File::create(format!("uploads/{}", &filename)).await.unwrap();

    // Process the file payload
    while let Some(chunk) = payload.next().await {
        let data = chunk.unwrap();
        file.write_all(&data).await.unwrap();
    }

    Ok(HttpResponse::Ok().body(format!("File uploaded: {}", filename)))
}

async fn send_email() -> Result<(), Error> {
    let smtp_host = std::env::var("SMTP_HOST").unwrap();
    let smtp_port: u16 = std::env::var("SMTP_PORT").unwrap().parse().unwrap();
    let smtp_username = std::env::var("SMTP_USERNAME").unwrap();
    let smtp_password = std::env::var("SMTP_PASSWORD").unwrap();

    let email = lettre::Message::builder()
        .from(smtp_username.parse().unwrap())
        .to(smtp_username.parse().unwrap())
        .subject("Test Email")
        .body("This is a test email.")
        .unwrap();

    let credentials = Credentials::new(smtp_username.clone(), smtp_password.clone());
    let mailer = SmtpClient::new_simple(&smtp_host)
        .unwrap()
        .credentials(credentials)
        .transport();

    mailer.send(&email).unwrap();

    Ok(())
}

async fn shutdown(server: Server) {
    println!("Shutting down server...");
    server.stop(true).await;
    println!("Server stopped");
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = PgPool::connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .expect("Failed to connect to the database.");

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone()) // Share the database connection pool across threads
            .route("/", web::get().to(index)) // Handle GET requests at the root path
            .route("/upload", web::post().to(upload)) // Handle POST requests to /upload
            .route("/static/{filename:.*}", web::get().to(|| {
                let filename: String = req.match_info().query("filename").parse().unwrap();
                let path = format!("static/{}", filename);
                async move {
                    match NamedFile::open(&path) {
                        Ok(file) => Ok(file),
                        Err(_) => Ok(HttpResponse::NotFound().body("File not found")),
                    }
                }
            })) // Serve static files from the /static directory
            .route("/send_email", web::get().to(send_email)) // Handle GET requests to /send_email
    })
    .bind("127.0.0.1:8080")?
    .shutdown_timeout(10)
    .run();

    println!("Server running at http://localhost:8080");

    // Graceful shutdown
    tokio::spawn(async move {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to listen for Ctrl+C");
        shutdown(server).await;
    });

    System::new().block_on(server)?;

    Ok(())
}
