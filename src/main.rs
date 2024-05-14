use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct MyRequest {
    name: String,
}

#[derive(Debug, Serialize)]
struct MyResponse {
    message: String,
}

#[post("/")]
async fn index_post(data: web::Json<MyRequest>) -> impl Responder {
    let name = &data.name; // Access the name field from the JSON request

    // Create a response with a message containing the received name
    let response = MyResponse {
        message: format!("Hello, {}! (POST)", name),
    };

    HttpResponse::Ok().json(response) // Return the response as JSON
}

#[derive(Deserialize)]
struct QueryParams {
    name: String,
}

#[get("/")]
async fn index_get(params: web::Query<QueryParams>) -> impl Responder {
    let name = &params.name; // Access the 'name' query parameter

    // Create a response with a message containing the received name
    let response = MyResponse {
        message: format!("Hello, {}! (GET)", name),
    };

    HttpResponse::Ok().json(response) // Return the response as JSON
}

// #[actix_web::main]
#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index_post) // Handle POST requests at the root path
            .service(index_get)  // Handle GET requests at the root path
    })
    .bind(("0.0.0.0", 8181))?
    .run()
    .await
}
