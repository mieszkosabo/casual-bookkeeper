use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: String,
    pub name: String
}

impl Member {
    pub fn new(name: String) -> Member {
        Member {
            id: Uuid::new_v4().to_string(),
            name
        }
    }
}