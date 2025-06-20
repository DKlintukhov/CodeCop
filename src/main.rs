use dotenv::dotenv;
use std::env;

fn main() {
    dotenv().ok();

    let key = env::var("API_KEY").expect("API_KEY must be set");
    let url = env::var("BASE_URL").expect("BASE_URL must be set");
    println!("ENV: {key} {url}");
}
