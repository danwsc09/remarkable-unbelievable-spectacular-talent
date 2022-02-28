use std::env;

use command_line_rust::Config;

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("ARGS: {:?}", args);
    let config = Config::new(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments: {}", err);
        std::process::exit(1);
    });
    
    println!("Searching for {}", config.query);
    println!("In file {}", config.filename);
    
    if let Err(e) = command_line_rust::run(config) {
        println!("Application Error: {}", e);
        std::process::exit(1);
    }
}
