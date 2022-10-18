
use rocket::{main, uri};

use models::response::Message;
use controllers::hello_controller;
use services::hello_service::HelloService;
use crate::app::build_rocket;

mod app;

pub mod controllers {
    pub mod hello_controller;
    pub mod controller;
}

pub mod services {
    pub mod hello_service;
    pub mod service;
}

pub mod repositories {
    pub mod repository;
    pub mod hello_repository;
}

pub mod models {
    pub mod response;
}

type RocketLaunch = Result<(), rocket::Error>;

#[main]
async fn main() -> RocketLaunch {
    println!("Hello, world!");
    let rocket = build_rocket();
    let  _ = rocket.launch().await?;
    Ok(())
}