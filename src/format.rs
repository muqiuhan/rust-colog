use crate::color;
use chrono::{Datelike, Local, Timelike};
use colored::*;
use env_logger::fmt::Formatter;
use log::Level;
use log::Record;
use std::io::{Error, Write};

fn level_token(level: &Level) -> &str {
    match *level {
        Level::Error => "!",
        Level::Warn => "#",
        Level::Info => "*",
        Level::Debug => "?",
        Level::Trace => "-",
    }
}

fn prefix_token(level: &Level) -> String {
    format!(
        "{}{}{} {}",
        "[".blue().bold(),
        color::level_color(level, level_token(level)),
        "]".blue().bold(),
        color::level_color(level, ">")
    )
}

pub fn format(buf: &mut Formatter, record: &Record<'_>) -> Result<(), Error> {
    let now = Local::now();
    let date_str = format!(
        "{}.{}.{:02} {}:{}:{:03}",
        now.day(),
        now.month(),
        now.year(),
        now.hour(),
        now.minute(),
        now.timestamp_subsec_micros() / 1000
    );

    let sep = format!("\n{}{} ", " ".repeat(date_str.len() + 2), " |  ".white());

    writeln!(
        buf,
        "{} {} {}",
        date_str.cyan(),
        prefix_token(&record.level()),
        format!("{}", record.args()).replace("\n", &sep),
    )
}
