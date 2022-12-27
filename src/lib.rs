use std::{env::args, process::exit};
use checked_command::{CheckedCommand, Error};
use openai_api::{self, api::Engine, api::CompletionArgs};

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

pub async fn openai_results(prompt: String) -> String {
    println!("Prompt:\n{}", prompt);
    let api_token = std::env::var("OPENAI_SK").expect("Open API Not found");
    let client = openai_api::Client::new(&api_token);
    let mut new_builder = openai_api::api::CompletionArgs::builder();
    let builder = new_builder
        .prompt(prompt.as_str())
        .engine(Engine::Davinci)
        .max_tokens(1000)
        .temperature(0.7);
    let args: CompletionArgs = builder.build().expect("msg");
    let completion = client.complete_prompt(args).await.expect("Completion not found");
    completion.choices[0].text.clone()
}
