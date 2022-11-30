use std::any::{TypeId};
use std::collections::HashMap;
use std::sync::{Arc};
use crate::provider::{Component, Provider};

pub trait Repository: Component {}

pub mod hello_repository;
pub mod tracker_repository;

#[derive(Clone)]
pub struct RepositoryProvider {
    set: HashMap<TypeId, Arc<dyn Component>>
}

impl Provider for RepositoryProvider {
    type ProviderImpl = Self;
    fn identity(self) -> Self::ProviderImpl { self }
    fn store<T: Component>(&mut self, key: TypeId, value: Arc<T>) {
        self.set.insert(key, value);
    }
    fn retrieve(&self, key: &TypeId) -> Option<&Arc<dyn Component>> {
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