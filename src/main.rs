//mod print;
// mod vars;
//mod types;
mod strings;
//mod tuples;
// mod arrays;
// mod vectors;
//mod conditionals;
// mod loops;
//mod functions;
//mod structs;
//mod forenheat;
//mod bankaccountsystem;
// mod references;
// mod ownership;
//mod borrowing;
// mod calculator;
//mod todos_api;
//mod todos_api;
//mod parallel_download;


fn main() {
//     println!("Hello, world!");
//     print::run();
//     vars::run();
//    types::run();
      strings::run();
//    tuples::run();
//   arrays::run();
//   vectors::run();
//   conditionals::run();
//   loops::run();
//    functions::run();
//   structs::run();
//   forenheat::run();
//  bankaccountsystem::run();
// references::run();
// ownership::run();
// borrowing::run();
// calculator::run();
}


// #[tokio::main]
// async fn main() {
//     println!("🌐 Calling async fetch_posts from todos_api...");

//     if let Err(e) = todos_api::fetch_posts().await {
//         eprintln!("❌ Error fetching posts: {}", e);
//     }

//     println!("🌐 Calling posts by ID (1–10)...");
//     if let Err(e) = todos_api::fetch_posts_by_id().await {
//         eprintln!("❌ Error: {}", e);
//     }

//     println!("⬇️ Starting parallel file downloads...");
//     if let Err(e) = parallel_download::run().await {
//         eprintln!("❌ Download error: {}", e);
//     }
// }

// mod scheduler;
// use tokio::time::{sleep, Duration};

// #[tokio::main]
// async fn main() {
//     // Start the time-based scheduler
//     scheduler::start_scheduler().await;

//     // Your app can continue running other tasks here
//     loop {
//         println!("Main thread app is running...");
//         sleep(Duration::from_secs(5)).await;
//     }
// }


// API Implementations

// mod basic_apis;

// use axum::Router;
// use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() {
//     let app = Router::new().nest("/", basic_apis::routes());

//     let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     println!("🚀 Server running on http://{}", listener.local_addr().unwrap());

//     axum::serve(listener, app).await.unwrap();
// }

