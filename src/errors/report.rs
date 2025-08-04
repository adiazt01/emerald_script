pub fn report(line: &usize, message: &str, where_: &str) {
    // TODO Make this more flexible with the user, and don't kill the session
    eprintln!("Error: {} at line {}: {}", where_, line, message);
    std::process::exit(1);  
}