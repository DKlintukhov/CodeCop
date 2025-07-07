/*
 *  This file is part of nzbget. See <https://github.com/DKlintukhov/CodeCop>.
 *
 *  Copyright (C) 2025 Denis <denis.klintukhov@gmail.com>
 *
 *  This program is free software; you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation; either version 2 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

mod app_config;
mod prompts;

use dotenv::dotenv;
use openai::{
    Credentials,
    chat::{ChatCompletion, ChatCompletionMessage, ChatCompletionMessageRole},
};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let credentials = Credentials::from_env();

    let messages = vec![
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::System,
            content: Some("You are a helpful assistant.".to_string()),
            ..Default::default()
        },
        ChatCompletionMessage {
            role: ChatCompletionMessageRole::User,
            content: Some("Tell me a random fact".to_string()),
            ..Default::default()
        },
    ];
    let chat_completion = ChatCompletion::builder("openrouter/cypher-alpha:free", messages.clone())
        .credentials(credentials.clone())
        .create()
        .await
        .unwrap();
    let returned_message = chat_completion.choices.first().unwrap().message.clone();
    println!(
        "{:#?}: {}",
        returned_message.role,
        returned_message.content.unwrap().trim()
    );
}
