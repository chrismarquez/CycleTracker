use crate::controllers::controller::Controller;
use crate::models::response::{Tracker, ID, Error};
use crate::services::tracker_service::TrackerService;
use rocket::serde::json::Json;
use rocket::{get, put, routes, Route, State, delete};
use rocket::response::status::NotFound;
use crate::models::request::TrackerRequest;

pub struct TrackerController {
    base: String,
}

impl TrackerController {
    pub fn new() -> Self {
        Self {
            base: "/tracker".to_string(),
        }
    }
}

impl Into<Vec<Route>> for TrackerController {
    fn into(self) -> Vec<Route> {
        routes![get_tracker, get_all_tracker, create_tracker, delete_tracker]
    }
}

impl Controller for TrackerController {
    fn get_base(&self) -> String {
        self.base.clone()
    }
}

#[get("/<tracker_id>")]
async fn get_tracker(tracker_service: &State<TrackerService>, tracker_id: i32) -> Result<Json<Tracker>, NotFound<Json<Error>>> {
    let tracker = tracker_service.get(tracker_id).await;
    match tracker {
        Some(tracker) => Ok(Json(tracker)),
        None => Err(
            NotFound(
                Json(Error::new(format!("The tracker with ID : {} was not found", tracker_id)))
            )
        )
    }
}

#[get("/")]
async fn get_all_tracker(tracker_service: &State<TrackerService>) -> Json<Vec<i32>> {
    let id_list = tracker_service.get_all_id().await;
    Json(id_list)
}

#[put("/<tracker_id>", data = "<tracker>")]
async fn create_tracker(
    tracker_service: &State<TrackerService>,
    tracker_id: i32,
    tracker: Json<TrackerRequest>,
) -> Json<ID> {
    let result = tracker_service.put(tracker_id, tracker.into_inner()).await;
    match result {
        Some(id) => Json(ID { id }),
        None => Json(ID { id: -1 }),
    }
}


#[delete("/<tracker_id>")]
async fn delete_tracker(
    tracker_service: &State<TrackerService>,
    tracker_id: i32,
) -> Json<ID> {
    let result = tracker_service.delete(tracker_id).await;
    match result {
        Some(id) => Json(ID { id }),
        None => Json(ID { id: -1 }),
    }
}
