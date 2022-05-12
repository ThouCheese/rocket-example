use crate::db;
use crate::model::NewPost;
use rocket::routes;
use rocket::serde::json::Json;

pub fn routes() -> Vec<rocket::Route> {
    routes![create_post]
}

#[derive(serde::Deserialize)]
pub struct CreatePostCommand {
    content: String,
}

impl Into<crate::model::NewPost> for CreatePostCommand {
    fn into(self) -> crate::model::NewPost {
        crate::model::NewPost {
            content: self.content,
        }
    }
}

#[derive(serde::Serialize)]
pub struct ContentDto {
    content: String,
    length: usize,
}

impl From<crate::model::Post> for ContentDto {
    fn from(model: crate::model::Post) -> Self {
        let length = model.content.len();
        Self {
            content: model.content,
            length,
        }
    }
}

#[rocket::post("/", data = "<command>")]
async fn create_post(command: Json<CreatePostCommand>, conn: db::Db) -> Json<ContentDto> {
    let command: NewPost = command.into_inner().into();
    conn.run(move |c| Json(command.create(c).into())).await
}
