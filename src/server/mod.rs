pub mod http;
pub mod response;
pub mod routes;

pub use http::HttpServer;
pub use response::Response;
pub use routes::Routes;

/// Server runtime entry for optional HTTP serving.
#[derive(Clone, Debug, Default)]
pub struct Server {
    pub http: HttpServer,
}

impl Server {
    pub fn new(port: u16) -> Self {
        Self { http: HttpServer::new(port, true) }
    }
}
