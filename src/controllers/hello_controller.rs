use crate::controllers::controller::Controller;
use crate::{HelloService, Message};
use rocket::serde::json::Json;
use rocket::{get, routes, Route, State};

pub struct HelloController {
    base: String,
}

impl HelloController {
    pub fn new() -> Self {
        Self {
            base: "/hello".to_string(),
        }
    }
}

impl Into<Vec<Route>> for HelloController {
    fn into(self) -> Vec<Route> {
        routes![index]
    }
}

impl Controller for HelloController {
    fn get_base(&self) -> String {
        self.base.clone()
    }
}

#[get("/<item>")]
fn index(hello_service: &State<HelloService>, item: &str) -> Json<Message> {
    let status = hello_service.update_status(item);
    let message = Message::new(status);
    Json(message)
}
