use crate::repositories::repository::Repository;

pub struct HelloRepository {
    items: Vec<String>
}

impl HelloRepository {

    pub fn new() -> Self {
        Self {
            items: Vec::new()
        }
    }

    pub fn add(&mut self, data: String) -> String {
        self.items.push(data.clone());
        data
    }

    pub fn exists(&self, data: String) -> bool {
        self.items.contains(&data)
    }

}

impl Repository for HelloRepository {}