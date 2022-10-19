use rocket::{Build, Rocket};
use crate::controllers::controller::Controller;
use crate::controllers::tracker_controller::TrackerController;

use crate::hello_controller::HelloController;
use crate::HelloService;
use crate::repositories::hello_repository::HelloRepository;

use crate::services::tracker_service::TrackerService;

trait RocketApp<'t> {
    fn manage_services(self) -> Rocket<Build>;
    fn mount_controllers(self) -> Rocket<Build>;
    fn mount_controller(self, controller: impl Controller<'t>) -> Rocket<Build>;
}

impl<'t> RocketApp<'t> for Rocket<Build> {
    fn manage_services(self) -> Rocket<Build> {
        self
            .manage(
                HelloService::new(
                    HelloRepository::new()
                )
            )
            .manage(TrackerService::new())
    }

    fn mount_controllers(self) -> Rocket<Build> {
        self
            .mount_controller(HelloController::new())
            .mount_controller(TrackerController::new())
    }

    fn mount_controller(self, controller: impl Controller<'t>) -> Rocket<Build> {
        self.mount(controller.get_base(), controller)
    }
}

pub fn build_rocket() -> Rocket<Build> {
    rocket::build()
        .manage_services()
        .mount_controllers()
}