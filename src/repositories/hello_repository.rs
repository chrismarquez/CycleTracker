use tokio::sync::RwLock;
use crate::provider::Component;
use crate::repositories::Repository;

pub struct HelloRepository {
    items: RwLock<Vec<String>>
}

impl HelloRepository {

    pub(in crate::repositories) fn new() -> Self {
        Self {
            items: RwLock::new(Vec::new())
        }
    }

    pub async fn add(&self, data: String) -> String {
        let mut writer = self.items.write().await;
        writer.push(data.clone());
        data
    }

    pub async fn exists(&self, data: String) -> bool {
        let reader = self.items.read().await;
        reader.contains(&data)
    }

}

impl Component for HelloRepository {}
impl Repository for HelloRepository {}