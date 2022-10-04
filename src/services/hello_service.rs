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
        let rep: &RefCell<HelloRepository> = self.repository.borrow();
        if rep.exists(item.to_string()) {
            item.to_string()
        } else {
            let cell = self.repository.borrow();
            cell.borrow_mut().exists(item.to_string())
        }
    }

}

impl Service for HelloService {}