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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn create_tracker() {
        let tracker_v1 = TrackerVersion::V1;
        let test_tracker = Tracker::new(65535, "testing tracker name".to_string(), tracker_v1);
        assert_eq!(test_tracker.id, 65535);
        assert_eq!(test_tracker.name, "testing tracker name");
        assert_eq!(test_tracker.version, TrackerVersion::V1);
    }
}