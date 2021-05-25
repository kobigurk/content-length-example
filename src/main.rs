fn main() {
  let mut rt =         tokio::runtime::Builder::new()
            .threaded_scheduler()
            .enable_all()
            .build()
            .unwrap();
  rt.block_on(async {
    let client = reqwest::Client::new();
    let result = client.head(&"https://plumophase2testing.blob.core.windows.net/chunks/0.100.0".to_string()).send().await.unwrap().error_for_status().unwrap();
    println!("content length: {:?}", result.headers()["content-length"].to_str().unwrap().parse::<u64>());
  });
}
