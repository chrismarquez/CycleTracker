use std::borrow::Borrow;
use crate::repositories::repository::HelloRepository;
use crate::services::service::Service;

pub struct HelloService {
    repository: Box<HelloRepository>
}

impl HelloService {

    pub fn new(repository: Box<HelloRepository>) -> Self {
        Self { repository }
    }

    pub fn update_status(&self, item: &str) -> String {
        todo!("If not there, update. If there, return")
    }

}

impl Service for HelloService {}