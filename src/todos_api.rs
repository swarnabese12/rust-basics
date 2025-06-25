use reqwest;
use serde::Deserialize;
use futures::future::join_all;

#[derive(Debug, Deserialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub body: String,
}

pub async fn fetch_posts() -> Result<(), Box<dyn std::error::Error>> {
    let url = "https://jsonplaceholder.typicode.com/posts";
    let response = reqwest::get(url).await?;

    let posts: Vec<Post> = response.json().await?;

    println!("📦 Fetched {} posts", posts.len());
    for post in posts.iter().take(5) {
        println!("\n🔹 Post ID: {}\nTitle: {}\nBody: {}", post.id, post.title, post.body);
    }
    

    Ok(())
}

// HTTP requests launched one by one
pub async fn fetch_posts_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    for id in 1..=10 {
        let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
        let response = client.get(&url).send().await?;

        let post: Post = response.json().await?;

        println!("\n📌 Post {} ---> {:?}", id, post);
        // println!("Title: {}", post.title);
        // println!("Body: {}\n", post.body);
    }

    Ok(())
}

//  HTTP requests launched at the same time
//join_all awaits all of them together (like Promise.all).
pub async fn fetch_posts_by_id() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();

    // Create a list of futures
    let fetches = (1..=10).map(|id| {
        let client = client.clone(); // clone client for each future
        async move {
            let url = format!("https://jsonplaceholder.typicode.com/posts/{}", id);
            let response = client.get(&url).send().await?;
            let post: Post = response.json().await?;
            Ok::<_, reqwest::Error>((id, post))
        }
    });

    // Run all futures concurrently
    let results = join_all(fetches).await;

    // Handle the results
    for result in results {
        match result {
            Ok((id, post)) => {
                println!("\n📌 Post {} ---> {:?}", id, post);
            }
            Err(err) => {
                eprintln!("❌ Error fetching post: {}", err);
            }
        }
    }

    Ok(())
}
