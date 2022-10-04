use rocket::Route;

pub trait Controller<'t>: Into<Vec<Route>> {
    fn get_base(&self) -> &'t str;
}