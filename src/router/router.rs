use axum::{
    routing::{delete, get, post, put, patch}, // Import patch here
    Router as AxumRouter,
    body::Body, // Import Body
    http::Method,
    handler::Handler,
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

    // A generic method to add routes based on HTTP method
    fn route<H>(mut self, method: Method, path: &str, handler: H) -> Self
    where
        H: Handler<Body, ()> + Clone + Send + Sync + 'static,
    {
        self.axum_router = match method {
            Method::GET => self.axum_router.route(path, get(handler)),
            Method::POST => self.axum_router.route(path, post(handler)),
            Method::PUT => self.axum_router.route(path, put(handler)),
            Method::PATCH => self.axum_router.route(path, patch(handler)),
            Method::DELETE => self.axum_router.route(path, delete(handler)),
            _ => panic!("Unsupported method"), // Handle unsupported methods if necessary
        };
        
        self.routes.insert(path.to_string(), method.to_string());
        self
    }

    // Convenience methods for each HTTP method
    pub fn get<H>(self, path: &str, handler: H) -> Self
    where
        H: Handler<Body, ()> + Clone + Send + Sync + 'static,
    {
        self.route(Method::GET, path, handler)
    }

    pub fn post<H>(self, path: &str, handler: H) -> Self
    where
        H: Handler<Body, ()> + Clone + Send + Sync + 'static,
    {
        self.route(Method::POST, path, handler)
    }

    pub fn put<H>(self, path: &str, handler: H) -> Self
    where
        H: Handler<Body, ()> + Clone + Send + Sync + 'static,
    {
        self.route(Method::PUT, path, handler)
    }

    pub fn patch<H>(self, path: &str, handler: H) -> Self
    where
        H: Handler<Body, ()> + Clone + Send + Sync + 'static,
    {
        self.route(Method::PATCH, path, handler)
    }

    pub fn delete<H>(self, path: &str, handler: H) -> Self
    where
        H: Handler<Body, ()> + Clone + Send + Sync + 'static,
    {
        self.route(Method::DELETE, path, handler)
    }

    pub fn build(self) -> AxumRouter {
        self.axum_router
    }
}