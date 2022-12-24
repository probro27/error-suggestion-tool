use code_errors;

fn main() {
    let command = code_errors::get_command();
    let result = code_errors::execute_command(command);
    println!("Result: Code: {}, error: {}", result.exit_code, result.error);
}
