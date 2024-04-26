#[tokio::main]
async fn main() {
    let client = reqwest::Client::new();
    loop {
        let resp = client
            .post("http://127.0.0.1:8000")
            .json(&vec![0; 1024 * 1024 * 10])
            .send()
            .await
            .unwrap()
            .text()
            .await
            .unwrap();
        println!("{resp}")
    }
}
