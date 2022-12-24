use std::env::args;
use std::process::Command;

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
    let mut args = args().collect::<Vec<String>>()[1..].iter().map(|element| String::from(element)).collect::<Vec<String>>();
    if args.len() != 0 {
        args.remove(0);
    }
    let program = std::env::args().collect::<Vec<String>>()[1].clone();
    CommandExecute { 
        program: program, 
        args: args 
    }
}

pub fn execute_command(command: CommandExecute) -> CommandResult {
    let mut program = Command::new(command.program.clone());
    program.args(command.args);
    println!("Command: {:?}", program);
    let exit_status = program.status().expect(&format!("Could not run the command")[..]);
    if exit_status.success() {
        CommandResult { 
            exit_code: 0, 
            error: String::from("Ran successfully") 
        }
    } else {
        CommandResult { 
            exit_code: 1, 
            error: String::from("Failure")
        }
    }
    
}
