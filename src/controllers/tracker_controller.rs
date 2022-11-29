use rocket::{get, put, Route, routes, State};
use rocket::serde::json::Json;
use crate::controllers::controller::Controller;
use crate::models::response::Tracker;
use crate::services::TrackerService;


pub struct TrackerController {
    base: String
}

impl TrackerController {
    pub fn new() -> Self {
        Self {
            base: "/tracker".to_string()
        }
    }
}

impl Into<Vec<Route>> for TrackerController {
    fn into(self) -> Vec<Route> { routes![get_tracker, get_all_tracker] }
}

impl Controller for TrackerController {
    fn get_base(&self) -> String { self.base.clone() }
}

#[get("/<tracker_id>")]
async fn get_tracker(tracker_service: &State<TrackerService>, tracker_id: i32) -> Json<Tracker> {
    let tracker = tracker_service.get(tracker_id).await;
    match tracker {
        Some(tracker) => {
            Json(tracker)
        }
        None => {
            todo!()
        }
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
    tracker: Json<Tracker>
) -> Json<Tracker> {
    todo!()
}