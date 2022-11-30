use std::sync::Arc;
use rocket::{State};

pub type Managed<T> = State<Arc<T>>;