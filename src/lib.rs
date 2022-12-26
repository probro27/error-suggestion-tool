use std::{env::args, process::exit};
use checked_command::{CheckedCommand, Error};

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
        _ => panic!("How is this possible")
    }
}

pub async fn openai_results(prompt: String) -> String {
    String::new()
}
// pub async fn chatgpt_results(prompt: String) -> String {
//     let token = std::env::var("CHATGPT_COOKIE").unwrap();
//     let mut client = ChatGPT::new(token).expect("Token in wrong");
//     client.refresh_token().await.expect("could not refersh");
//     // create a new empty conversation
//     // let mut conversation = client.new_conversation();
//     let response = client.send_message(&prompt[..]).await;
//     let send_response = response.expect("Error receiving response");
//     // expect("Error receiving response");
//     send_response
// }
