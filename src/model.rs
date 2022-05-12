use crate::schema::posts;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub content: String,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost {
    pub content: String,
}

impl NewPost {
    pub fn create(self, conn: &diesel::PgConnection) -> Post {
        diesel::insert_into(posts::table)
            .values(&self)
            .get_result(conn)
            .unwrap()
    }
}
