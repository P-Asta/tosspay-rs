use tosspay::TossPay;
#[tokio::main]
async fn main() {
    let toss = TossPay::new("objective".to_string());
    toss.pay(1000).await;
}
