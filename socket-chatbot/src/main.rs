use std::time::Duration;

use anyhow::{anyhow, Result};
use async_openai::{
    types::{
        ChatCompletionRequestMessage, ChatCompletionRequestSystemMessageArgs,
        ChatCompletionRequestUserMessageArgs, CreateChatCompletionRequestArgs,
    },
    Client,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::timeout,
};

#[tokio::main]
async fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Server running on 127.0.0.1:8080");

    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            println!("New user connected");
            match user_loop(&mut socket).await {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }
            println!("User disconnected");
        });
    }
}

enum Input
{
    Empty,
    Question(String),
}

impl From<Option<String>> for Input
{
    fn from(input: Option<String>) -> Self
    {
        match input
        {
            Some(input) => Input::Question(input),
            None => Input::Empty,
        }
    }
}

async fn get_user_input(socket: &mut TcpStream) -> Result<Input> {
    let mut buffer = [0; 1024];
    let n = match timeout(Duration::from_secs(30), socket.read(&mut buffer)).await? {
        Ok(0) => return Err(anyhow!("Connection closed")),
        Ok(1) if buffer[0] == b'\n' => return Ok(None.into()),
        Ok(2) if buffer[0] == b'\r' && buffer[1] == b'\n' => return Ok(None.into()),
        Ok(n) => n,
        Err(e) => return Err(anyhow!("Failed to read from socket: {}", e)),
    };

    let input = String::from_utf8_lossy(&buffer[..n]);

    Ok(Some(input.into_owned()).into())
}

async fn user_loop(socket: &mut TcpStream) -> Result<()> {
    let client = Client::new();

    let prompt = tokio::fs::read_to_string("prompt.txt").await?;

    let mut messages: Vec<ChatCompletionRequestMessage> =
        vec![ChatCompletionRequestSystemMessageArgs::default()
            .content(prompt)
            .build()?
            .into()];

    loop {
        let input = match get_user_input(socket).await? {
            Input::Question(input) => input,
            Input::Empty => return Ok(()),
        };

        messages.push(
            ChatCompletionRequestUserMessageArgs::default()
                .content(input.to_string())
                .build()?
                .into(),
        );

        let request = CreateChatCompletionRequestArgs::default()
            .model("gpt-4o")
            .messages(messages.clone())
            .build()?;

        let response = client
            .chat() // Get the API "group" (completions, images, etc.) from the client
            .create(request) // Make the API call in that "group"
            .await?;

        let output = response
            .choices
            .first()
            .unwrap()
            .message
            .content
            .as_ref()
            .unwrap();

        socket.write_all(output.as_bytes()).await.unwrap();
        socket.write_all(b"\r\n").await.unwrap();
    }
}
