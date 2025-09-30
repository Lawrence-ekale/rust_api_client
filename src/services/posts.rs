use crate::{models::Post, error::AppResult };

pub async fn get_posts() -> AppResult<Vec<Post>> {
    let response = reqwest::get("https://jsonplaceholder.typicode.com/posts")
        .await?
        .json::<Vec<Post>>()
        .await?;

    Ok(response)
}

pub fn filter_posts_by_user(posts: Vec<Post>,user_id: i32) -> Vec<Post> {
    posts.into_iter()
        .filter(|p| p.user_id == user_id)
    .collect()
}

pub async fn get_posts_by_user_id(user_id: i32) -> AppResult<Vec<Post>> {
    let url = format!("https://jsonplaceholder.typicode.com/posts?userId={}", user_id);
    let response = reqwest::get(&url)
        .await?
    .json::<Vec<Post>>()
        .await?;

    Ok(response)
}