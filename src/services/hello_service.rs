use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::Arc;
use crate::repositories::repository::HelloRepository;
use crate::services::service::Service;

pub struct HelloService {
    repository: Arc<RefCell<HelloRepository>>
}

impl HelloService {

    pub fn new(repository: Arc<RefCell<HelloRepository>>) -> Self {
        Self { repository }
    }

    pub fn update_status(&self, item: &str) -> String {
        todo!("If not there, update. If there, return")
    }

}

impl Service for HelloService {}