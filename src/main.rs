#[rocket::main]
async fn main() {
    let _ = rocket_example::rocket().launch().await;
}
