use std::{env::args, process::exit};
use checked_command::{CheckedCommand, Error};
use http::{HeaderMap, HeaderValue, header::{AUTHORIZATION, CONTENT_TYPE}};
use reqwest::Body;
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize)]
struct Choice {
    text: String,
    index: i32,
    logprobs: Option<String>,
    finish_reason: String
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct Usage {
    prompt_tokens: i32,
    completion_tokens: i32,
    total_tokens: i32
}

#[allow(dead_code)]
#[derive(Deserialize)]
struct OpenAIResponse {
    id: String,
    object: String,
    created: i32,
    model: String,
    choices: Vec<Choice>,
    usage: Usage
}

#[derive(Debug, Clone)]
pub struct CommandExecute {
    program: String,
    args: Vec<String>
}

pub struct CommandResult {
    pub exit_code: i32,
    pub error: String
}

pub fn get_command() -> CommandExecute {
    let args = args().collect::<Vec<String>>();
    if args.len() == 1 {
        exit(1)
    }
    let mut args = args[1..].iter().map(|element| String::from(element)).collect::<Vec<String>>();
    if args.len() != 0 {
        args.remove(0);
    }
    let program = std::env::args().collect::<Vec<String>>()[1].clone();
    CommandExecute { 
        program, 
        args 
    }
}

pub fn execute_command(command: CommandExecute) -> CommandResult {
    let mut program = CheckedCommand::new(command.program.clone());
    program.args(command.args);
    println!("Command: {:?}", program);
    let result = program.output();
    match result {
        Ok(output) => CommandResult { 
            exit_code: 0, 
            error: String::from_utf8_lossy(&*output.stdout).to_string()
        },
        Err(Error::Failure(ex, output)) => {
            let mut error_message = String::new();
            if let Some(out) = output {
                error_message += &format!("{}",
                String::from_utf8_lossy(&*out.stderr))[..];
                CommandResult {
                    exit_code: ex.code().unwrap_or_default(),
                    error: error_message
                }
            } else {
                CommandResult { 
                    exit_code: ex.code().unwrap_or_default(),
                    error: String::from("Failure")
                }
            }
        },
        _ => panic!("How is this possible {:?}", result)
    }
}

pub async fn openai_results_fetch(prompt: String) -> String {
    let api_token = std::env::var("OPENAI_SK").expect("Open API Not found");
    let url = String::from("https://api.openai.com/v1/completions");
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_str("application/json").unwrap());
    headers.insert(AUTHORIZATION, HeaderValue::from_str(format!("Bearer {}", api_token).as_str()).unwrap());
    let mut body = json::JsonValue::new_object();
    body["model"] = "text-davinci-003".into();
    body["prompt"] = prompt.into();
    body["max_tokens"] = 1000.into();
    let body_str = body.dump();
    let res = client
                .post(url)
                .headers(headers)
                .body(Into::<Body>::into(body_str.clone()))
                .send()
                .await
                .expect("Response not received");
    let res_json = res.json::<OpenAIResponse>().await.expect("Error deseralizing to json");
    res_json.choices[0].text.clone()
}
