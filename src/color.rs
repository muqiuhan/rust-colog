use colored::*;
use log::Level;

#[allow(dead_code)]
pub fn level_color(level: &log::Level, msg: &str) -> String {
    match level {
        Level::Error => msg.red().blink(),
        Level::Warn => msg.yellow().blink(),
        Level::Info => msg.green(),
        Level::Debug => msg.blue(),
        Level::Trace => msg.magenta(),
    }
    .bold()
    .to_string()
}
