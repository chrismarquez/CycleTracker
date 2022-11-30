use rocket::{get, Route, routes};
use rocket::serde::json::Json;
use crate::services::HelloService;
use crate::controllers::controller::Controller;
use crate::Message;
use crate::types::Managed;

pub struct HelloController {
    base: String
}

impl HelloController {
    pub fn new() -> Self {
        Self {
            base: "/hello".to_string(),
        }
    }
}

impl Into<Vec<Route>> for HelloController {
    fn into(self) -> Vec<Route> { routes![index] }
}

impl Controller for HelloController {
    fn get_base(&self) -> String { self.base.clone() }
}

#[get("/<item>")]
async fn index(hello_service: &Managed<HelloService>, item: &str) -> Json<Message> {
    let status = hello_service.update_status(item).await;
    let message = Message::new(status);
    Json(message)
}