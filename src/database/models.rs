use diesel::prelude::*;
use uuid::Uuid;
use diesel::sql_types::Uuid as DieselUuid;
use chrono::{DateTime, Utc};

#[derive(Queryable, Identifiable, Debug, Associations)]
#[diesel(belongs_to(User, foreign_key = id_user))]
#[diesel(table_name = crate::schema::posts)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub id_user: Uuid,
    pub created_at: DateTime<Utc>,
    pub height: i32,
    pub version: String,
    pub width: i32,
    pub url_bucket: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub id_user: Uuid,
    pub height: i32,
    pub version: String,
    pub width: i32,
    pub url_bucket: String,
}

#[derive(Queryable, Identifiable, Debug, Associations)]
#[diesel(belongs_to(User, foreign_key = id_user))]
#[diesel(belongs_to(Post, foreign_key = id_post))]
#[diesel(table_name = crate::schema::comments)]
pub struct Comment {
    pub id: Uuid,
    pub id_user: Uuid,
    pub id_post: Uuid,
    pub content: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::comments)]
pub struct NewComment {
    pub id_user: Uuid,
    pub id_post: Uuid,
    pub content: String,
}

#[derive(Queryable, Identifiable, Debug, Associations)]
#[diesel(belongs_to(User, foreign_key = id_user))]
#[diesel(belongs_to(Post, foreign_key = id_post))]
#[diesel(table_name = crate::schema::ratings)]
pub struct Rating {
    pub id: Uuid,
    pub value: i32,
    pub id_user: Uuid,
    pub id_post: Uuid,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::ratings)]
pub struct NewRating {
    pub value: i32,
    pub id_user: Uuid,
    pub id_post: Uuid,
    pub description: Option<String>,
}


#[derive(Queryable, Identifiable, Debug)]
#[diesel(table_name = crate::schema::users)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
pub struct NewUser {
    pub name: String,
    pub email: String,
    pub password: String,
}