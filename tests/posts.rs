use rocket::{http::Status, local::blocking::Client};
use serde_json::Value;

#[test]
fn test_create_post() {
    let client = Client::tracked(rocket_example::rocket()).unwrap();
    let body: Value = serde_json::from_str(r#"{"content": "some super nice content"}"#).unwrap();
    let req = client.post("/posts").json(&body);
    let resp = req.dispatch();
    assert_eq!(resp.status(), Status::Ok, "{:?}", resp.into_string());
    let expected: Value = serde_json::from_str(
        r#"{
            "content": "some super nice content",
            "length": 23
        }"#,
    )
    .unwrap();
    let actual: Value = resp.into_json().unwrap();
    assert_eq!(actual, expected);
}
