use rocket::serde::{Deserialize, Serialize};

pub mod request;
pub mod response;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub enum TrackerVersion {
    V1,
}
