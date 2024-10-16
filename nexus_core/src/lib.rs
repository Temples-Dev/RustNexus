use std::net::SocketAddr;

use nexus_router::Router;
use nexus_error::AppError;
use std::fs;


pub struct NexusApp {
    pub router: Router,
}

impl NexusApp {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }


    pub fn discover_and_register_routes(&mut self) {
        let routes_dir = "src/routes";
        if let Ok(entries) = fs::read_dir(routes_dir) {
            for entry in entries.flatten() {
                if let Some(file_name) = entry.file_name().to_str() {
                    if file_name.ends_with(".rs") {
                        // This is where we'd parse the file and register the routes
                        // For now, we'll use a placeholder that needs to be replaced with actual parsing and registration
                        println!("Would parse and register routes from: {}", file_name);
                    }
                }
            }
        }
    }



    pub async fn run(self, addr: SocketAddr)-> Result<(), AppError> {
        let routes = self.router.get_routes();
        for (path, method) in routes {
            tracing::info!("Registered route: {} {}", method, path);
        }

        let axum_router = self.router.build();
        
        tracing::info!("Listening on {}", addr);
        let listener = tokio::net::TcpListener::bind(addr).await.map_err(|_| AppError::BindError(addr))?;
        axum::serve(listener, axum_router).await.map_err(AppError::Io)?;

        Ok(())
    }
}