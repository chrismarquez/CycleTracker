use rocket::Route;

pub trait Controller: Into<Vec<Route>> {
    fn get_base(&self) -> String;
}