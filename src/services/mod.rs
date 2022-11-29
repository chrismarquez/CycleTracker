use std::any::{Any, TypeId};
use std::borrow::Borrow;
use std::collections::HashMap;
use std::ops::Deref;
use std::sync::Arc;
use downcast_rs::impl_downcast;
use rocket::get;
use crate::models::response::Tracker;
use crate::provider;
use crate::provider::{Component, Provider};
use crate::repositories::{HelloRepository, RepositoryProvider as RepoProvider};

pub mod hello_service;
pub mod tracker_service;

impl_downcast!(sync Service);
pub trait Service: Component {}

#[derive(Clone)]
pub struct ServiceProvider {
    set: HashMap<TypeId, Arc<dyn Component>>
}

impl Provider for ServiceProvider {
    type ProviderImpl = Self;
    fn identity(self) -> Self::ProviderImpl { self }
    fn store<T: Component>(&mut self, key: TypeId, value: Arc<T>) {
        self.set.insert(key, value);
    }
    fn retrieve(&self, key: &TypeId) -> Option<&Arc<dyn Component>> {
        self.set.get(key)
    }
}

impl ServiceProvider {
    pub async fn new() -> Self {
        let provider = RepoProvider::new().await;
        Self { set: HashMap::new() }
            .manage(HelloService::new(provider.get()))
            .manage(TrackerService::new(provider.get()))
    }
}

pub use self::{
    tracker_service::TrackerService,
    hello_service::HelloService
};