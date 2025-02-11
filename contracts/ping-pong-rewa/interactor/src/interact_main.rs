extern crate ping_pong_rewa_interact;

#[tokio::main]
pub async fn main() {
    ping_pong_rewa_interact::ping_pong_rewa_cli().await;
}
