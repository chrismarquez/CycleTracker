use rocket::{get, Route, routes, State};
use rocket::serde::json::Json;
use crate::controllers::controller::Controller;
use crate::models::response::Tracker;
use crate::services::tracker_service::TrackerService;

pub struct TrackerController<'t> {
    base: &'t str
}

impl<'t> TrackerController<'t> {
    pub fn new() -> Self {
        Self {
            base: "/tracker"
        }
    }
}

impl<'t> Into<Vec<Route>> for TrackerController<'t> {
    fn into(self) -> Vec<Route> { routes![get_tracker, get_all_tracker] }
}

impl<'t> Controller<'t> for TrackerController<'t> {
    fn get_base(&self) -> &'t str { self.base }
}

#[get("/<tracker_id>")]
fn get_tracker(tracker_service: &State<TrackerService>, tracker_id: &str) -> Json<Tracker> {
    let tracker = tracker_service.get(tracker_id.to_string());
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
fn get_all_tracker(tracker_service: &State<TrackerService>) -> Json<Vec<u8>> {
    let id_list = tracker_service.get_all_id();
    Json(id_list)
}