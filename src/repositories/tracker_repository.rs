use std::error;
use std::fmt;
use std::fmt::{Debug, Display, Formatter};
use crate::models::response::{Tracker, ID};
use crate::repositories::repository::Repository;

use mongodb::{Client, Collection, options::ClientOptions};
use mongodb::bson::doc;
use mongodb::error::{Error as MongoError, ErrorKind};
use mongodb::options::FindOptions;
use mongodb::results::InsertOneResult;
use rocket::futures::TryStreamExt;
use rocket::serde::json::Json;
use crate::repositories::tracker_repository::RepositoryError::NotFound;


pub struct TrackerRepository {
    collection: Collection<Tracker>,
    id_collection: Collection<ID>
}

#[derive(Debug)]
enum RepositoryError {
    Connection(MongoError),
    NotFound(String)
}

impl Display for RepositoryError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            RepositoryError::Connection(ref err) => match *err.kind.as_ref() {
                ErrorKind::InvalidArgument { ref message, .. } => write!(f, "Invalid Arg {}", message),
                _ => write!(f, "Unknown Database Error"),
            }
            NotFound(ref message) => write!(f, "Element Not Found: {}", message)
        }
    }
}

impl error::Error for RepositoryError {}

impl From<MongoError> for RepositoryError {
    fn from(err: MongoError) -> Self {
        Self::Connection(err)
    }
}

impl TrackerRepository {
    pub async fn new() -> Self {
        match TrackerRepository::_new().await {
            Ok(repo) => repo,
            Err(collection) => panic! { "{}", collection },
        }
    }

    async fn _new() -> Result<Self, RepositoryError> {
        let mut client_options = ClientOptions::parse("mongodb://localhost:27017").await?;
        client_options.app_name = Some("My App".to_string());
        let client = Client::with_options(client_options)?;
        let repo = Self {
            collection: client.database("test").collection("Tracker"),
            id_collection: client.database("test").collection("Tracker")
        };
        Ok(repo)
    }

    pub async fn get(&self, id: i32) -> Option<Tracker> {
        let query = doc! { "id": &id };
        if let Ok(tracker) = self.collection.find_one(query, None).await {
            return tracker
        }
        None
    }


    pub async fn get_all(&self) -> Vec<i32> {
        let fields = doc! { "id": 1 };
        let options = FindOptions::builder()
            .projection(fields)
            .build();
        if let Ok(cursor) = self.id_collection.find(None, options).await {
            if let Ok(results) = cursor.try_collect::<Vec<ID>>().await {
                let id_list: Vec<i32> = results.into_iter().map(|it| { it.id }).collect();
                return id_list
            }
        }
        Vec::new()
    }

    pub async fn put(&self, id:i32, tracker: Tracker) -> Option<i32> {
        if let Ok(result) =  self.collection.insert_one(tracker, None).await {
            return result.inserted_id.as_i32()
        }
        None
    }
}

impl Repository for TrackerRepository {}