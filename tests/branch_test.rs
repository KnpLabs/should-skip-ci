mod utils;

use std::env::current_dir;
use std::path::{
    Path,
    PathBuf,
};
use std::fs::read_to_string;
use rand;
use ssc::should_skip_ci;
use utils::{
    create_remote_repo,
    create_local_repo,
    set_remote_repo,
    create_and_push_initial_commit,
    checkout_new_branch,
    append_content_to_api_readme,
    commit_and_push_changes,
    create_api_test_file,
};

#[test]
fn it_should_return_success_code() {
    assert_eq!(0, should_skip_ci(
        &current_dir().unwrap(),
        &vec![PathBuf::from(".")],
        &String::from("true"),
        &String::from("origin"),
        &String::from("master"),
    ))
}

#[test]
fn it_should_detect_changes_on_branch() {
    // Setup test case

    let rdm = rand::random::<u32>();
    let base_path = PathBuf::from(format!(
        "/tmp/ssc-functional-test-{}",
        rdm,
    ));
    let remote_repo_path = create_remote_repo(&base_path);
    let local_repo_path = create_local_repo(&base_path);

    set_remote_repo(&local_repo_path, &remote_repo_path);

    create_and_push_initial_commit(&local_repo_path);

    let branch_name = String::from("test/new-branch");
    checkout_new_branch(&local_repo_path, &branch_name);

    append_content_to_api_readme(&local_repo_path);

    commit_and_push_changes(
        &local_repo_path,
        &branch_name,
        &String::from("first commit"),
    );

    create_api_test_file(&local_repo_path);

    commit_and_push_changes(
        &local_repo_path,
        &branch_name,
        &String::from("second commit"),
    );

    // Run should-skip-ci and make assertions

    let mut api_app_path = PathBuf::from(&local_repo_path);
    api_app_path.push("apps/api");

    let mut front_app_path = PathBuf::from(&local_repo_path);
    front_app_path.push("apps/front");

    let witness_filename = "witness.txt";
    let witness_content = "stop command executed";
    let mut witness_filepath = PathBuf::from(&base_path);
    witness_filepath.push(&witness_filename);

    let cmd = format!(
        "echo -n \"{}\" > {}",
        &witness_content,
        &witness_filepath.to_str().unwrap(),
    );

    // assert that the file created by the stop command is not here before using
    // should-skip-ci
    assert_eq!(false, Path::new(&witness_filepath).exists());

    // should not skip the CI as we made changes on the api app
    should_skip_ci(
        &local_repo_path,
        &vec![api_app_path],
        &cmd,
        &String::from("origin"),
        &String::from("master"),
    );

    // the stop command should not have been ran
    assert_eq!(false, Path::new(&witness_filepath).exists());

    // should skip the CI as we did not make any changes on the front app
    should_skip_ci(
        &local_repo_path,
        &vec![front_app_path],
        &cmd,
        &String::from("origin"),
        &String::from("master"),
    );

    // the stop command should have been ran
    assert_eq!(true, Path::new(&witness_filepath).exists());

    let actual_content = read_to_string(&witness_filepath)
        .expect("Unable to read witness file content")
    ;

    assert_eq!(&actual_content, &witness_content);
}
