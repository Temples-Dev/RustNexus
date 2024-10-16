use std::net::SocketAddr;
use super::router::Router;
use super::error::AppError;

pub struct App {
    pub router: Router,
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
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