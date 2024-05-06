use std::future::Future;
use std::pin::Pin;
use tide::Request;
use std::collections::HashMap;
use std::time::{Duration};
use serde::{Deserialize, Serialize};
#[macro_use] extern crate lazy_static;
use std::sync::Mutex;

pub mod structs;
pub mod task_handler;
pub mod task_utils;

const KEEPIE_SERVER: &str = "http://localhost:8000";
const CURREN_SERVER: &str = "localhost:8081";
#[derive(Debug, Deserialize, Serialize)]
struct Secret {
    username: String,
    password: String,
}
#[derive(Debug, Deserialize, Serialize)]
struct AppRequest {
    receive_url: String
}

lazy_static! {
    static ref SESSION: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
}
fn auth_middleware<'a>( mut req: tide::Request<()>, next: tide::Next<'a, ()>)
    -> Pin<Box<dyn Future<Output = tide::Result> + 'a + Send>> {
    let token: String  = uuid::Uuid::new_v4().to_string();
    let cred = base64::encode(format!("{}{}{}", "salt".to_owned(),
                                      req.header("username").unwrap().last().to_owned(),
                                      req.header("password").unwrap().last().to_owned()));
    let receive_url: String = format!("http://{}/givemesecret?token={}&cred={}", CURREN_SERVER.to_owned(), token.to_owned(), cred.to_owned());

    Box::pin(async move {
        let mut authenticated = false;
        for _ in 0..10 {

            let mut receive_url_2 = String::new();
            receive_url_2.push_str(&receive_url);

            let data = AppRequest {
                receive_url: receive_url_2
            };
            surf::post(format!("{}/sendSecretToMe", KEEPIE_SERVER))
                .body_json(&data)
                .unwrap()
                .await
                .unwrap();

            match SESSION.lock().unwrap().get(&token) {
                None => {
                    std::thread::sleep(Duration::new(1, 0));
                },
                Some(username) => {
                    req.append_header("username", username.to_owned());
                    authenticated = true;
                    break
                }
            }
        }

        if authenticated {
            SESSION.lock().unwrap().remove(&token).expect("failed to remove session");
            Ok(next.run(req).await)
        } else {
            Ok(tide::Response::new(tide::StatusCode::Unauthorized))
        }
    })
}


#[async_std::main]
async fn main() -> tide::Result<()> {
    // init db file
    task_utils::init();

    let mut app = tide::new();

    app.at("/").post(|_| async { Ok("ok")} );
    app.at("/add_task").with(auth_middleware).post(task_handler::add_task);
    app.at("/get_task").with(auth_middleware).post(task_handler::get_task);
    app.at("/list_task").with(auth_middleware).post(task_handler::list_task);
    app.at("/delete_task").with(auth_middleware).post(task_handler::remove_task);
    app.at("/modify_task").with(auth_middleware).post(task_handler::modify_task);

    app.at("/givemesecret").post( |mut req: Request<()>| async move {
        #[derive(Deserialize)]
        struct CredAndToken { cred: String, token: String }
        let Secret { username, password } = req.body_json().await?;
        let page: CredAndToken = req.query()?;

        // check credencials
        if page.cred != base64::encode(format!("{}{}{}", "salt".to_owned(), username.to_owned(), password.to_owned())) {
            return Ok(tide::Response::new(tide::StatusCode::Unauthorized));
        }
        SESSION.lock().unwrap().insert(page.token.to_owned(), username.to_owned());
        return Ok("ok".into());
    });

    app.listen("127.0.0.1:8081").await?;
    Ok(())
}
