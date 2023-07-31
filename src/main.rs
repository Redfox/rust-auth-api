use std::net::SocketAddr;

mod app;

#[tokio::main]
async fn main() {
    let app = app::create_app().await;

    let port = 3333;
    let address = SocketAddr::from(([0, 0, 0, 0], port));

    println!("Server running on port {}", port);

    axum::Server::bind(&address)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");

}
