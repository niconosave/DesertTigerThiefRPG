use std::process::ExitCode;

fn main() -> ExitCode {
    match desert_tiger_terminal::ejecutar(std::env::args()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("Error: {error}");
            ExitCode::FAILURE
        }
    }
}
