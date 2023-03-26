use std::sync::{Arc};
use crate::repositories::HelloRepository;
use crate::services::Service;

pub struct HelloService {
    repository: Arc<HelloRepository>
}

impl HelloService {

    pub(in crate::services) fn new(repository: Arc<HelloRepository>) -> Self {
        Self { repository }
    }

    pub async fn update_status(&self, item: &str) -> String {
        if self.is_cached(&item).await {   // is this good enough? can we handle this in a better way?
            item.to_string()
        } else {
            self.repository.add(item.to_string()).await
        }
    }

    async fn is_cached(&self, item: &str) -> bool {
        self.repository.exists(item.to_string()).await
    }

}

impl Service for HelloService {}
