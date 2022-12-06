use crate::models::TrackerVersion;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Message {
    status: String,
}

impl Message {
    pub fn new(status: String) -> Self {
        Self { status }
    }
}


#[derive(Serialize, Deserialize)]
pub struct Tracker {
    pub id: i32,
    pub name: String,
    pub version: TrackerVersion,
}

#[derive(Serialize, Deserialize)]
pub struct ID {
    pub id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    pub message: String
}


impl Tracker {
    pub fn new(id: i32, name: String, version: TrackerVersion) -> Self {
        Self {
            id,
            name,
            version
        }
    }
}

impl Error {
    pub fn new(message: String) -> Self {
        Self {
            message
        }
    }
}