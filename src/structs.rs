use tide::prelude::*;

#[derive(Debug, Deserialize, Serialize)]
pub struct Task {
    pub id: u32,
    pub name: String,
    pub completed: bool,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct TaskNoId {
    pub name: String,
    pub completed: bool,
}
