use crate::server::Server;

/// `serve` command handler.
#[derive(Clone, Debug)]
pub struct ServeCommand {
    pub port: u16,
}

impl ServeCommand {
    pub fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn run(&self) -> String {
        let server = Server::new(self.port);
        let addr = server.http.bind();
        format!("Server listening on {}", addr)
    }
}
