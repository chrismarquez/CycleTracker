use rocket::{Build, Rocket, async_trait};
use crate::controllers::controller::Controller;
use crate::controllers::tracker_controller::TrackerController;

use crate::hello_controller::HelloController;
use crate::HelloService;
use crate::repositories::hello_repository::HelloRepository;
use crate::repositories::tracker_repository::TrackerRepository;

use crate::services::tracker_service::TrackerService;

#[async_trait]
trait RocketApp {
    async fn manage_services(self) -> Rocket<Build>;
    fn mount_controllers(self) -> Rocket<Build>;
    fn mount_controller(self, controller: impl Controller) -> Rocket<Build>;
}

#[async_trait]
impl RocketApp for Rocket<Build> {
    async fn manage_services(self) -> Rocket<Build> {
        self
            .manage(
                HelloService::new(
                    HelloRepository::new()
                )
            )
            .manage(TrackerService::new(
                TrackerRepository::new().await
            ))
    }

    fn mount_controllers(self) -> Rocket<Build> {
        self
            .mount_controller(HelloController::new())
            .mount_controller(TrackerController::new())
    }

    fn mount_controller(self, controller: impl Controller) -> Rocket<Build> {
        self.mount(controller.get_base(), controller)
    }
}

pub async fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .manage_services().await
        .mount_controllers()
}