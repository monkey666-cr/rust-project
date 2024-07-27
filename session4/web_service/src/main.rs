use axum::response::Html;
use axum::{routing::get, Router};
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_text))
        .route("/json", get(say_hello_json));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_text() -> Html<&'static str> {
    Html("<h1>Hello World!</h1>")
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    let message = HelloJson {
        message: "Hi from JSON".to_string(),
    };

    axum::Json(message)
}
