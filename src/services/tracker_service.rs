use std::sync::RwLock;
use crate::models::response::Tracker;
use crate::repositories::tracker_repository::TrackerRepository;
use crate::services::service::Service;

pub struct TrackerService {
    repository: RwLock<TrackerRepository>
}

impl TrackerService {

    pub fn new(repository: TrackerRepository) -> Self {
        Self {
            repository: RwLock::new(repository)
        }
    }

    pub fn get(&self, id: String) -> Option<Tracker> {
        let repo = self.repository.read().expect("Ups");
        repo.get(id)
    }

    pub fn get_all_id(&self) -> Vec<u8> {
        todo!()
    }

}

impl Service for TrackerService {}