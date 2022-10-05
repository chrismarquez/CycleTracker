use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use crate::repositories::hello_repository::HelloRepository;
use crate::services::service::Service;

pub struct HelloService {
    repository: RwLock<HelloRepository>
}

impl HelloService {

    pub fn new(repository: HelloRepository) -> Self {
        Self {
            repository: RwLock::new(repository)
        }
    }

    pub fn update_status(&self, item: &str) -> String {
        let repo = self.repository.read()
            .expect("Failed to read");
        if repo.exists(item.to_string()) {
            item.to_string()
        } else {
            let mut repo = self.repository.write()
                .expect("Failed to write");
            repo.add(item.to_string())
        }
    }

}

impl Service for HelloService {}