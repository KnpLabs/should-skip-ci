use std::process::Command;

pub fn run_stop_cmd(cmd: &String) -> i32 {
    // As the cmd to run is a single string, we run it though a shell
    // otherwise rust will look for an executable which *is* the whole
    // `cmd` string.
    let result = Command::new("/bin/sh")
        .arg("-c")
        .arg(cmd)
        .output()
        .expect("Unable to run the stop command.")
    ;

    return result.status.code().unwrap();
}
