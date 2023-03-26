use std::collections::HashMap;

impl_downcast!(sync Repository);
pub trait Repository: DowncastSync {}

pub mod hello_repository;
pub mod tracker_repository;
use provider::{AutoProvide};
use crate::repositories::{AutoProvider as X};

#[derive(AutoProvide, Clone)]
#[component(Repository)]
pub struct RepositoryProvider {
    set: HashMap<TypeId, Arc<dyn Repository>>
}

impl AutoProvider for RepositoryProvider {
    type ProviderImpl = Self;
    fn identity(self) -> Self::ProviderImpl { self }
    fn store<T: Repository>(&mut self, key: TypeId, value: Arc<T>) {
        self.set.insert(key, value);
    }
    fn retrieve(&self, key: &TypeId) -> Option<&Arc<dyn Repository>> {
        self.set.get(key)
    }
}

impl RepositoryProvider {
    pub async fn new() ->  Self {
        Self { set: HashMap::new() }
            .manage(HelloRepository::new())
            .manage(TrackerRepository::new().await)
    }
}

pub use self::{
    hello_repository::HelloRepository,
    tracker_repository::TrackerRepository
};
