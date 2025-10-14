const USAGE: &'static str = "
    Minimal HTTP client for exploring HTTP, HTTPS (and TLS)

    Usage: 
        httpr [METHOD] <URL> [OPTIONS]

    Args: 
        [METHOD]    The HTTP method to use. Defaults to GET.
        <URL>       The target URL. Supports HTTP and HTTPS.

    Options:
        -H, --header <name:value>   Adds a request header. Repeatable.
        -d, --data <STRING>         Adds a request body.
        -o, --output <FILE>         save response to file
        ....
";

pub fn print_usage() {
    println!("{USAGE}");    
}