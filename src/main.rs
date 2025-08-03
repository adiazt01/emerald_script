use std::{env, fs, io::{self, stdin, BufRead, Write}};

mod lexer;

use lexer::token_types;

fn main() {
    let args = env::args().collect::<Vec<String>>();

    if args.len() < 2 {
        println!("Usage: emerald scripts [script]");
    } else if args.len() == 1 {
        // Run a file
        let path = &args[0];
        run_file(path);
    } else {
        run_prompt();
    }
}

fn run_file(path: &str) {
    let bytes = fs::read(path)
        .expect("Failed to read the file");

    let script = String::from_utf8(bytes)
        .expect("Failed to convert bytes to string");

    run(&script);
}



fn run_prompt() {
    // Prompt for input
    let mut reader = stdin().lock();

    loop {
        print!("> ");
        // Flush stdout to ensure prompt appears before input
        io::stdout().flush().unwrap();
    
        // Read a line from stdin
        let mut line = String::new();

        // Read a line from the reader
        // This will block until a line is read or EOF is reached
        let bytes_read = reader.read_line(&mut line).unwrap();

        // Check if we reached EOF
        if bytes_read == 0 {
            // EOF reached, exit the loop
            break;
        }

        let line = line.trim_end();

        if line.is_empty() {
            continue; // Skip empty lines
        }
    
        // Run the line as a script
        run(line);
    }
}

fn run(source: &str) {
    // Scan the input source
    println!("{}", source);
}

fn error(line: &u128, message: &str) {
    report(line, message, "Error");
}

fn report(line: &u128, message: &str, where_: &str) {
    // TODO Make this more flexible with the user, and don't kill the session
    eprintln!("Error: {} at line {}: {}", where_, line, message);
    std::process::exit(1);  
}