use dotenv::dotenv;
use openai::{
    Credentials,
    chat::{ChatCompletionMessage, ChatCompletionMessageRole},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let credentials = Credentials::from_env();

    let messages = vec![ChatCompletionMessage {
        role: ChatCompletionMessageRole::System,
        content: Some("You are a large language model built into a command line interface as an example of what the `openai` Rust library made by Valentine Briese can do.".to_string()),
        ..Default::default()
    }];
    println!("{:?}, {:?}", credentials, messages);
}
