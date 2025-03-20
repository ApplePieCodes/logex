use colored::Colorize;

pub fn log_info(s: &str) {
    println!("{} {}", "[INFO]:".bold().bright_white(), s);
}
pub fn log_warning(s: &str) {
    println!("{} {}", "[WARNING]:".bold().yellow(), s);
}
pub fn log_error(s: &str) {
    println!("{} {}", "[ERROR]:".bold().bright_red(), s);
}
pub fn log_fatal_error(s: &str) {
    println!("{} {}", "[FATAL ERROR]:".bold().red(), s);
    panic!("{}", s);
}

#[cfg(test)]
mod tests {
    use galvanic_test::test_suite;

    test_suite! {
        name logging_tests;
        use crate::{log_info, log_warning, log_error, log_fatal_error};

        #[should_panic]
        test info_test() {
            log_info("I am info");
            log_warning("I am warning");
            log_error("I am error");
            log_fatal_error("I, am steve");
        }
    }
}