extern crate log;

use std::process::Command;
use log::debug;

pub fn run_stop_cmd(cmd: &String) -> i32 {
    // As the cmd to run is a single string, we run it though a shell
    // otherwise rust will look for an executable which *is* the whole
    // `cmd` string.
    let mut command = Command::new("/bin/sh");
    command
        .arg("-c")
        .arg(cmd)
    ;

    debug!("Running `{:?}`", command);

    let result = command
        .output()
        .expect("Unable to run the stop command.")
    ;


    return result.status.code().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cmd_exit_code_is_returned() {
        assert_eq!(0, run_stop_cmd(&String::from("true")));

        assert_eq!(1, run_stop_cmd(&String::from("false")));
    }
}
