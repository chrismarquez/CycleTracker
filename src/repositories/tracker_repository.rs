use crate::models::response::Tracker;
use crate::repositories::repository::Repository;

use mongodb::{Client, options::ClientOptions};


pub struct TrackerRepository {
    client: Client
}

impl TrackerRepository {

    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, id: String) -> Option<Tracker> {
        todo!("")
    }

    pub fn get_all(&self) -> Vec<u8> {
        todo!("")
    }
}

impl Repository for TrackerRepository {}