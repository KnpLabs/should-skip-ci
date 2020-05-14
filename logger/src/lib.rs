extern crate log;
extern crate simple_logger;

use log::Level;

pub fn configure_logger(verbosity: &u8) {
    let level: Level;

    match *verbosity {
        1 => level = Level::Info,
        2 => level = Level::Debug,
        _ => level = Level::Warn,
    }

    simple_logger::init_with_level(level).unwrap();
}
