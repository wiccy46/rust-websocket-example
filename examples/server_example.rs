use example_websocket::start_server;

#[tokio::main]
async fn main() {
    start_server("127.0.0.1:9001").await;
}