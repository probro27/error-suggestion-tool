use code_errors;
extern crate dotenv;

use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let command = code_errors::get_command();
    let result = code_errors::execute_command(command);
    print!("Result: Code: {}, error/output:\n{}", result.exit_code, result.error);
    let prompt = String::from("Provide suggestion for this error with suitable links:\n") + &result.error;
    // let response = code_errors::chatgpt_results(prompt).await;
    // print!("Solution is:\n {}", response);
}
