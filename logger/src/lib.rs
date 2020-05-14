extern crate log;
extern crate simple_logger;

use log::Level;

pub fn configure_logger(verbosity: &u8) {
    let mut level = Level::Warn;

    if 1 == *verbosity {
        level = Level::Info;
    }

    simple_logger::init_with_level(level).unwrap();
}
