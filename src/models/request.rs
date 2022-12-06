use crate::models::TrackerVersion;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct TrackerRequest {
    pub name: String,
    pub version: TrackerVersion,
}