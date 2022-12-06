extern crate core;

use rocket::main;

use crate::app::build_rocket;
use controllers::hello_controller;
use models::response::Message;
use services::hello_service::HelloService;

mod app;
mod models;

pub mod controllers {
    pub mod controller;
    pub mod hello_controller;
    pub mod tracker_controller;
}

pub mod services {
    pub mod hello_service;
    pub mod service;
    pub mod tracker_service;
}

pub mod repositories {
    pub mod hello_repository;
    pub mod repository;
    pub mod tracker_repository;
}

type RocketLaunch = Result<(), rocket::Error>;

#[main]
async fn main() -> RocketLaunch {
    println!("Hello, world!");
    let rocket = build_rocket().await;
    let _ = rocket.launch().await?;
    Ok(())
}
