use std::collections::HashMap;
use provider_ioc::{AutoProvide};
use crate::repositories::{AutoProvider as RepoAutoProvider, RepositoryProvider};

impl_downcast!(sync Service);
pub trait Service: DowncastSync {}

mod hello_service;
mod tracker_service;

#[derive(AutoProvide, Clone)]
#[component(Service)]
pub struct ServiceProvider {
    set: HashMap<TypeId, Arc<dyn Service>>
}

impl AutoProvider for ServiceProvider {
    type ProviderImpl = Self;
    fn identity(self) -> Self::ProviderImpl { self }
    fn store<T: Service>(&mut self, key: TypeId, value: Arc<T>) {
        self.set.insert(key, value);
    }
    fn retrieve(&self, key: &TypeId) -> Option<&Arc<dyn Service>> {
        self.set.get(key)
    }
}

impl ServiceProvider {
    pub async fn new() -> Self {
        let provider = RepositoryProvider::new().await;
        Self { set: HashMap::new() }
            .manage(HelloService::new(provider.get()))
            .manage(TrackerService::new(provider.get()))
    }
}

pub use self::{
    tracker_service::TrackerService,
    hello_service::HelloService
};
