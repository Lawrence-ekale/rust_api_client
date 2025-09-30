use std::io;
use std::io::Write;
use rust_api_client::error::AppResult;
use rust_api_client::services::posts::{get_posts, get_posts_by_user_id};

#[tokio::main]
async fn main() -> AppResult<()> {

    println!("The program filters posts by User Id");
    print!("Enter a user id (or press Enter to fetch all): ");
    io::stdout().flush().unwrap();//flush for user to print immediately

    let mut user_id_input = String::new();
    io::stdin().read_line(&mut user_id_input).unwrap();
    let user_id = user_id_input.trim();
    if user_id.is_empty() {
        let posts = get_posts().await?;
        println!("{:#?}", posts);
    } else {
        let user_id: i32 = user_id.parse().expect("Invalid a valid number");

        let posts = get_posts_by_user_id(user_id).await?;//Backend query

        println!("Posts by user {}: {:#?}",user_id, posts);
    }
    Ok(())
}


