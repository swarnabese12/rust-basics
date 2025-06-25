use chrono::{Local, NaiveTime};
use tokio::time::{sleep, Duration};

pub async fn start_scheduler() {
    tokio::spawn(async {
        let target_time = NaiveTime::parse_from_str("11:16", "%H:%M").unwrap();

        loop {
            let now = Local::now().naive_local();
            let current_time = now.time();

            println!("⏳ Checking time... Current time: {}", current_time.format("%H:%M:%S"));

            if current_time >= target_time && current_time < target_time + chrono::Duration::minutes(1) {
                println!("✅ Time matched! Running scheduled task...");
                send_message().await;

                // Wait till the next minute (to avoid multiple triggers)
                sleep(Duration::from_secs(60)).await;
            } else if current_time < target_time {
                // Difference in seconds until 11:16
                let seconds_left = (target_time - current_time).num_seconds();
                println!("🕒 Sleeping for {} seconds until 11:16...", seconds_left);
                sleep(Duration::from_secs(seconds_left as u64)).await;
            } else {
                // Already past 11:16 today — just sleep 60s and wait for next day
                println!("⏳ 11:16 already passed. Sleeping 60 seconds before checking again.");
                sleep(Duration::from_secs(60)).await;
            }
        }
    });
}

async fn send_message() {
    println!("⏰ Please add your LOGIN details");
}
