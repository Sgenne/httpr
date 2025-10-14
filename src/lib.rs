use crate::cli_error::CLIError;

pub mod cli_error;
mod io;

pub fn run(args: Vec<String>) -> Result<(), CLIError> {
    if args.is_empty() || args.len() == 1 {
        io::print_usage();
        return Ok(());   
    }

    println!("Commands!");
    Ok(())
}
