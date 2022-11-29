use std::sync::Arc;
use tokio::sync::RwLock;
use crate::models::response::Tracker;
use crate::provider::Component;
use crate::repositories::TrackerRepository;
use crate::services::Service;

pub struct TrackerService {
    repository: Arc<TrackerRepository>
}


impl TrackerService {

    pub(in crate::services) fn new(repository: Arc<TrackerRepository>) -> Self {
        Self { repository }
    }

    pub async fn get(&self, id: i32) -> Option<Tracker> {
        self.repository.get(id).await
    }

    pub async fn get_all_id(&self) -> Vec<i32> {
        self.repository.get_all().await
    }

}

impl Component for TrackerService {}
impl Service for TrackerService {}