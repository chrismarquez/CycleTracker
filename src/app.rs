use std::cell::RefCell;
use std::sync::Arc;
use rocket::{Build, Rocket};
use crate::controllers::controller::Controller;
use crate::hello_controller::HelloController;
use crate::HelloService;
use crate::repositories::hello_repository::HelloRepository;
use crate::services::service::Service;

fn get_services() -> Vec<impl Service> {
    vec!(
        HelloService::new(
            HelloRepository::new()
        )
    )
}

fn get_controllers<'t>() -> Vec<impl Controller<'t>> {
    vec!(
        HelloController::new()
    )
}

pub fn build_rocket() -> Rocket<Build> {
    let builder = rocket::build();
    let builder = get_services().into_iter()
        .fold(builder, |b, service| {
            b.manage(service)
        });

    get_controllers().into_iter()
        .fold(builder, |b, controller| {
            b.mount(controller.get_base(), controller)
        })
}