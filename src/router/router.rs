use axum::{
    routing::{delete, get, post, put},
    Router as AxumRouter,
};
use std::collections::HashMap;

pub struct Router {
    axum_router: AxumRouter,
    routes: HashMap<String, String>,
}

impl Router {
    pub fn new() -> Self {
        Router {
            axum_router: AxumRouter::new(),
            routes: HashMap::new(),
        }
    }

    pub fn get<H>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<(), Body = axum::body::Body> + Clone + Send + 'static,
    {
        self.axum_router = self.axum_router.route(path, get(handler));
        self.routes.insert(path.to_string(), "GET".to_string());
        self
    }

    pub fn post<H>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<(), Body = axum::body::Body> + Clone + Send + 'static,
    {
        self.axum_router = self.axum_router.route(path, post(handler));
        self.routes.insert(path.to_string(), "POST".to_string());
        self
    }

    pub fn put<H>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<(), Body = axum::body::Body> + Clone + Send + 'static,
    {
        self.axum_router = self.axum_router.route(path, put(handler));
        self.routes.insert(path.to_string(), "PUT".to_string());
        self
    }


    pub fn patch<H>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<(), Body = axum::body::Body> + Clone + Send + 'static,
    {
        self.axum_router = self.axum_router.route(path, patch(handler));
        self.routes.insert(path.to_string(), "PATCH".to_string());
        self
    }


    pub fn delete<H>(mut self, path: &str, handler: H) -> Self
    where
        H: axum::handler::Handler<(), Body = axum::body::Body> + Clone + Send + 'static,
    {
        self.axum_router = self.axum_router.route(path, delete(handler));
        self.routes.insert(path.to_string(), "DELETE".to_string());
        self
    }

    pub fn build(self) -> AxumRouter {
        self.axum_router
    }
}
