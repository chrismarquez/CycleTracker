
use rocket::{get, Route, routes, State};
use rocket::serde::json::Json;
use crate::{HelloService, Message};
use crate::controllers::controller::Controller;

pub struct HelloController<'t> {
    base: &'t str
}

impl<'t> HelloController<'t> {
    pub fn new() -> Self {
        Self {
            base: "/hello",
        }
    }
}

impl<'t> Into<Vec<Route>> for HelloController<'t> {
    fn into(self) -> Vec<Route> { routes![index] }
}

impl<'t> Controller<'t> for HelloController<'t> {
    fn get_base(&self) -> &'t str { self.base }
}

#[get("/<item>")]
fn index(hello_service: &State<HelloService>, item: &str) -> Json<Message> {
    let status = hello_service.update_status(item);
    let message = Message::new(status);
    Json(message)
}