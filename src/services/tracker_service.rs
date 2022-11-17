use tokio::sync::RwLock;
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

    pub async fn get(&self, id: i32) -> Option<Tracker> {
        let repo = self.repository.read().await;
        repo.get(id).await
    }

    pub async fn get_all_id(&self) -> Vec<i32> {
        let repo = self.repository.read().await;
        repo.get_all().await
    }

}

impl Service for TrackerService {}