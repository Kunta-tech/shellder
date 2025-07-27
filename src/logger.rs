// Copyright (c) 2025 Saugata Kundu - kundusaugata576@gmail.com
// Licensed under the Apache-2.0 License

use colored::*;

pub trait Logger {
    fn info(msg: &str);
    fn warn(msg: &str);
    fn error(msg: &str);
    fn success(msg: &str);
    fn log(msg: &str);
}
pub struct CliLogger;
impl Logger for CliLogger {
    fn info(msg: &str) {
        println!("{} {}", "[INFO ]".blue(), msg);
    }

    fn warn(msg: &str) {
        println!("{} {}", "[WARN ]".yellow(), msg);
    }

    fn error(msg: &str) {
        println!("{} {}", "[ERROR]".red(), msg);
    }

    fn success(msg: &str) {
        println!("{} {}", "[ OK  ]".green(), msg);
    }

    fn log(msg: &str) {
        println!("{} {}", "[ LOG ]".white(), msg);
    }
}
