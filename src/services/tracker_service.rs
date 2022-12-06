use crate::models::response::Tracker;
use crate::models::request::TrackerRequest;
use crate::repositories::tracker_repository::TrackerRepository;
use crate::services::service::Service;
use rocket::serde::json::Json;
use tokio::sync::RwLock;

pub struct TrackerService {
    repository: RwLock<TrackerRepository>,
}

impl TrackerService {
    pub fn new(repository: TrackerRepository) -> Self {
        Self {
            repository: RwLock::new(repository),
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

    pub async fn put(&self, id: i32, tracker_req: TrackerRequest) -> Option<i32> {
        let repo = self.repository.write().await;
        let tracker = Tracker::new(id, tracker_req.name, tracker_req.version);
        repo.put(tracker).await
    }

    pub async fn delete(&self, id: i32) -> Option<i32> {
        let repo = self.repository.write().await;
        repo.delete(id).await
    }
}

impl Service for TrackerService {}
