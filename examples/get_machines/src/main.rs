use futures::stream::{self, StreamExt};
use hackthebox_rs::HackTheBox;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let client = HackTheBox::new_authenticated(std::env::var("APP_TOKEN").unwrap());
    let mut futures = stream::iter((1..=5i32).map(|i| client.get_machine(i))).buffered(5);
    while let Some(m) = futures.next().await {
        println!("{}", m.unwrap());
    }
}
