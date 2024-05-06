use super::task_utils;
use super::structs::{Task, TaskNoId};
use tide::prelude::*;
use tide::Body;
use tide::Request;
use tide::Response;

#[derive(Debug, Deserialize, Serialize)]
struct RequestID {
    id: u32
}
pub async fn add_task(mut req: Request<()>) -> tide::Result {
    let TaskNoId {name, completed } = req.body_json().await?;
    task_utils::add(&TaskNoId{name: name, completed: completed});

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"status": "ok"}))?);
    Ok(res)
}

pub async fn list_task(mut _req: Request<()>) -> tide::Result {
    let data = task_utils::read_to_map();
    let data: Vec<&Task> = (&data).iter().map(|(_, task)| task).collect();

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"status": "ok", "data": data}))?);
    Ok(res)
}


pub async fn get_task(mut req: Request<()>) -> tide::Result {
    let RequestID { id } = req.body_json().await?;
    let data = task_utils::read_to_map();
    let data = data.get(&id);
    let mut res = Response::new(200);

    match data {
        Some(t) => {
            res.set_body(Body::from_json(&json!({"status": "ok", "data": t}))?);
        }
        None => {
            res.set_body(Body::from_json(&json!({"status": "ok", "data": None::<Task>}))?);
        }
    }
    Ok(res)

}

pub async fn remove_task(mut req: Request<()>) -> tide::Result {
    let RequestID { id } = req.body_json().await?;
    task_utils::remove_by_id(id);

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"status": "ok"}))?);
    Ok(res)
}


pub async fn modify_task(mut req: Request<()>) -> tide::Result {
    let task = req.body_json().await?;

    task_utils::modify(&task);

    let mut res = Response::new(200);
    res.set_body(Body::from_json(&json!({"status": "ok"}))?);
    Ok(res)
}

