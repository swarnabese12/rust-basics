use futures::future::join_all;
use reqwest;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::time::Instant;

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let urls = vec![
        ("https://upload.wikimedia.org/wikipedia/commons/4/47/PNG_transparency_demonstration_1.png", "image1.png"),
        ("https://www.w3.org/People/mimasa/test/imgformat/img/w3c_home.jpg", "image2.png"),
        ("https://via.placeholder.com/300x150.png?text=Hello", "image3.png"),
    ];

    println!("🚀 Starting parallel downloads...");
    let start = Instant::now();

    let tasks = urls.into_iter().map(|(url, filename)| {
        async move {
            match download_file(url, filename).await {
                Ok(_) => println!("✅ Downloaded: {}", filename),
                Err(e) => eprintln!("❌ Failed {}: {}", filename, e),
            }
        }
    });

    join_all(tasks).await;

    let duration = start.elapsed();
    println!("⏱️ All downloads completed in {:.2?}", duration);

    Ok(())
}

async fn download_file(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    let path = Path::new(filename);
    let mut file = File::create(path)?;
    file.write_all(&bytes)?;

    Ok(())
}
