use axum::{body::Body, extract::Request, response::Html, routing::get, Router};
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() {
    let file_appender = tracing_appender::rolling::hourly("test.log", "prefix.log");
    let (non_blocking, _guard) = tracing_appender::non_blocking(file_appender);

    let subscriber = tracing_subscriber::fmt()
        .compact()
        .json()
        .with_writer(non_blocking)
        .with_ansi(false)
        .with_file(true)
        .with_line_number(true)
        .with_thread_ids(true)
        .with_target(false)
        .with_span_events(FmtSpan::CLOSE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).unwrap();

    info!("Starting server");

    let app = Router::new()
        .route("/", get(|| async { "hello world" }))
        .route("/index", get(index))
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &Request<Body>| {
                let request_id = uuid::Uuid::new_v4();
                tracing::span!(
                    tracing::Level::INFO,
                    "request",
                    method = tracing::field::display(request.method()),
                    uri = tracing::field::display(request.uri()),
                    version = tracing::field::debug(request.version()),
                    request_id = tracing::field::display(request_id)
                )
            }),
        );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}

// #[instrument]
async fn index() -> Html<&'static str> {
    tracing::info!("this is info");
    tracing::debug!("this is debug");

    Html("<h1>Hello World</h1>")
}
