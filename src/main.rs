
/*
TODO: 
1. Add CLIContext for storing information about the requested operation throughout its execution
2. Add input parsing to io.rs
    * What should it return?
        * A CLIContext
3. There will be tons of types related to the content of a request. Collect these in a common folder directly under source. 
    * What should it be named? request_types? request_data? request_content?
*/

use std::env::args;

fn main() -> Result<(), httpr::cli_error::CLIError> {
    let args: Vec<String> = args().collect();
    httpr::run(args)    
}
