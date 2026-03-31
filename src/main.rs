fn main() {
    let start = std::time::Instant::now();

    for c in 0..3 {
        let _contents = reqwest::blocking::get("https://www.rust-lang.org")
            .unwrap()
            .text()
            .unwrap();

        let duration = start.elapsed();
        println!("{}件目の処理時間 {}ms", c, duration.as_millis());
    }
}