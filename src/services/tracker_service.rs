use crate::models::response::Tracker;
use crate::services::service::Service;

pub struct TrackerService {}

impl TrackerService {

    pub fn new() -> Self {
        Self {}
    }

    pub fn get(&self, id: String) -> Option<Tracker> {
        todo!()
    }

    pub fn get_all_id(&self) -> Vec<u8> {
        todo!()
    }

}

impl Service for TrackerService {}