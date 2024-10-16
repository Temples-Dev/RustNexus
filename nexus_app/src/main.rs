use std::net::SocketAddr;

use nexus_core::NexusApp;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let mut app = NexusApp::new();

    // Automatically discover and register all routes
    app.discover_and_register_routes();

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    if let Err(e) = app.run(addr).await {
        eprintln!("Server error: {}", e);
    }
}
