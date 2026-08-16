/// HTTP server stub for the single binary.
#[derive(Clone, Debug, Default)]
pub struct HttpServer {
    pub port: u16,
    pub offline: bool,
}

impl HttpServer {
    pub fn new(port: u16, offline: bool) -> Self {
        Self { port, offline }
    }

    pub fn bind(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }
}
