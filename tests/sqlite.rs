use rocket::local::asynchronous::Client;
use rocket_auth::*;
use serde_json::json;
mod common;
pub use common::client;
use serde_json::Value;

pub async fn client_sqlite(config: Option<Config>) -> Client {
    let users = Users::open_sqlite("tests/.test_database.sqlite")
        .await
        .unwrap();
    client(users, config).await
}
#[tokio::test]
async fn assert_logout() {
    let client = client_sqlite(None).await;
    let res = client.get("/").dispatch().await;
    let json: Value = res.into_json().await.unwrap();
    assert!(json["data"].is_null(), "{}", json);
}

macro_rules! assert_success {
    ($data:expr) => {
        let data: Value = $data;
        assert_eq!(
            data["status"],
            json!("success"),
            "expected success, found '{}'",
            data
        );
    };
}

#[tokio::test]
async fn happy_path() {
    let client = client_sqlite(None).await;
    let res = client.get("/csrf_token").dispatch().await;
    let json: Value = res.into_json().await.unwrap();
    let token = &json["csrf_token"];
    let payload = json!({
        "csrf_token": token,
        "email": "example@gmail.com",
        "password": "Password123"
    })
    .to_string();
    let data = client
        .post("/signup")
        .body(&payload)
        .dispatch()
        .await
        .into_json()
        .await
        .unwrap();
    assert_success!(data);
    let data = client
        .post("/login")
        .body(&payload)
        .dispatch()
        .await
        .into_json()
        .await
        .unwrap();
    assert_success!(data);
    let data = client
        .post("/logout")
        .body(json!({ "csrf_token": token }).to_string())
        .dispatch()
        .await
        .into_json()
        .await
        .unwrap();
    assert_success!(data);
    let data = client
        .post("/login")
        .body(&payload)
        .dispatch()
        .await
        .into_json()
        .await
        .unwrap();
    assert_success!(data);
    let data = client
        .delete("/delete")
        .dispatch()
        .await
        .into_json()
        .await
        .unwrap();
    assert_success!(data);
}
