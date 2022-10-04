use rocket::serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    status: String
}

impl Message {
    pub fn new(status: String) -> Self {
        Self { status }
    }
}