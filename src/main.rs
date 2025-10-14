
/*
TODO: 
2. Add input parsing to io.rs
3. Add CLIContext for storing information about the requested operation throughout its execution
*/

use std::env::args;

fn main() -> Result<(), httpr::cli_error::CLIError> {
    let args: Vec<String> = args().collect();
    httpr::run(args)    
}
