use code_errors;

fn main() {
    let command = code_errors::get_command();
    let result = code_errors::execute_command(command);
    print!("Result: Code: {}, error/output:\n{}", result.exit_code, result.error);
}
