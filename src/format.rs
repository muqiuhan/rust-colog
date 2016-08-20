use log::LogRecord;
use log::LogLevel;
use color;
use colored::*;

fn level_token(level: &LogLevel) -> &str
{
    match *level
    {
        LogLevel::Error => "E",
        LogLevel::Warn  => "W",
        LogLevel::Info  => "I",
        LogLevel::Debug => "D",
        LogLevel::Trace => "T",
    }
}

fn prefix_token(level: &LogLevel) -> String
{
    format!("{}{}{}", "[".blue().bold(), color::level_color(level, level_token(level)), "]".blue().bold())
}

pub fn format(record: &LogRecord) -> String
{
    let loc = record.location();
    format!(
        "{} args={}, loc.module_path={}, loc.file={}, loc.line={}, record.target={}",
        prefix_token(&record.level()),
        record.args(),
        loc.module_path(),
        loc.file(),
        loc.line(),
        record.target(),
    )
}
