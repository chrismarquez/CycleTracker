use rocket::serde::{Serialize, Deserialize};
use crate::models::response::TrackerVersion::V1;

#[derive(Serialize, Deserialize)]
pub struct Message {
    status: String
}

impl Message {
    pub fn new(status: String) -> Self {
        Self { status }
    }
}

#[derive(Serialize, Deserialize)]
pub enum TrackerVersion {
    V1
}

#[derive(Serialize, Deserialize)]
pub struct Tracker {
    id: u8,
    name: String,
    version: TrackerVersion
}

impl Tracker {
    pub fn new(id: u8, name: String) -> Self {
        Self { id, name, version: V1 }
    }
}