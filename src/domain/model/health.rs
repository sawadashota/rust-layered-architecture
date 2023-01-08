use serde::Serialize;

#[derive(Serialize)]
pub struct Health {
    pub status: String,
}

impl Health {
    pub fn ok() -> Health { Health { status: "ok".to_string() } }
}
