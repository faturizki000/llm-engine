/// Response serialization stub.
#[derive(Clone, Debug, Default)]
pub struct Response {
    pub status: u16,
    pub body: String,
}

impl Response {
    pub fn ok(body: impl Into<String>) -> Self {
        Self { status: 200, body: body.into() }
    }
}
