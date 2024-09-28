//! logex: a simple logger for rust apps

use colored::Colorize;

/// Type of log message
pub enum LogType {
    Info,
    Warning,
    Error,
    FatalError
}

/// Prints a log message to the screen in the format [c]: s
pub fn log(s: &str, c: LogType) {
    match c {
        LogType::Info => println!("{}: {}", "[INFO]".bold().bright_white(), s),
        LogType::Warning => println!("{}: {}", "[WARNING]".bold().yellow(), s),
        LogType::Error => println!("{}: {}", "[INFO]".bold().red(), s),
        LogType::FatalError => {
            println!("{}: {}", "[INFO]".bold().truecolor(139, 0, 0), s);
            panic!();
        }
    }
}