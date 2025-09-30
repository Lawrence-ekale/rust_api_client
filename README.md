# 🦀 Rust Posts API Client
This is my first step into the world of Rust programming.
I build a small command-line application that fetches posts from the [JSONPlaceholder API](https://jsonplaceholder.typicode.com/) and lets you filter them by userId.

The goal of this project is to:
- Learn <b>Rust's module system</b> (lib.rs, models, services, error).
- Practice <b>async programming</b> with [tokio](https://tokio.rs/).
- Use [reqwest](https://docs.rs/reqwest) for making HTTP requests.
- Explore <b>Serde</b> for JSON deserialization.
- Organize code in a <b>production-like structure</b> (separation of models, services, and error handling).
- Build a <b>simple CLI interface</b> for user interaction.

✨ Features
- Fetch all posts.
- Fetch posts for specific user by `userId`.
- interactive CLI prompt for `userId`.
- Error handling with a custom `AppError` type.
