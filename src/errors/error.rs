use crate::errors::report::report;

pub fn error(line: &usize, message: &str) {
    report(line, message, "Error");
}