use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub completed: bool,
}
