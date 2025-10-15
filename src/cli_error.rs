use std::fmt::Display;

#[derive(Debug)]
pub enum CLIError {

}

impl Display for CLIError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Something went wrong my duder")
    }
}

impl std::error::Error for CLIError {}