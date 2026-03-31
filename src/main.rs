#[tokio::main]
async fn main() {
    let start = std::time::Instant::now();
    let mut handles = vec![];
    for _ in 0..3 {
        let handle = tokio::spawn(async {
             let _contents = reqwest::get("https://www.rust-lang.org")
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }

    let duration = start.elapsed();
    println!("処理時間 {}ms", duration.as_millis());
}