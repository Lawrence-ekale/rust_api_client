use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Post {
    #[serde(rename = "userId")]
    pub user_id: i32,
    pub id: i32,
    pub title: String,
    pub body: String,
}