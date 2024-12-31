pub mod utils;

use std::path::{
    Path,
    PathBuf,
};
use std::fs::read_to_string;
use rand;
use ssc::should_skip_ci;
use ssc::git::commits_range::CommitsRange;
use utils::{
    open_repository_at_path,
    create_remote_repo,
    create_local_repo,
    set_remote_repo,
    create_and_push_initial_commit,
    checkout_new_branch,
    append_content_to_api_readme,
    commit_and_push_changes,
    create_api_test_file,
    checkout_branch,
    merge_given_branch_on_current_branch_non_fast_forward,
};

#[test]
fn it_should_detect_changes_on_a_merge_commit_on_master() {
    // Setup test case

    let rdm = rand::random::<u32>();
    let base_path = PathBuf::from(format!(
        "/tmp/ssc-functional-test-{}",
        rdm,
    ));
    let remote_repo_path = create_remote_repo(&base_path);
    let local_repo_path = create_local_repo(&base_path);

    set_remote_repo(&local_repo_path, &remote_repo_path);

    create_and_push_initial_commit(
        &local_repo_path,
        &String::from("master"),
    );

    let branch_name = String::from("test/new-branch");
    checkout_new_branch(&local_repo_path, &branch_name);

    append_content_to_api_readme(
        &local_repo_path,
        &String::from("\nmore content"),
    );

    commit_and_push_changes(
        &local_repo_path,
        &branch_name,
        &String::from("first commit"),
    );

    create_api_test_file(
        &local_repo_path,
        &String::from("test.txt"),
        &String::from("test content"),
    );

    commit_and_push_changes(
        &local_repo_path,
        &branch_name,
        &String::from("second commit"),
    );

    checkout_branch(
        &local_repo_path,
        &String::from("master"),
    );

    merge_given_branch_on_current_branch_non_fast_forward(
        &local_repo_path,
        &branch_name,
        &String::from("merge test branch into base branch"),
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

    let local_git_repo = open_repository_at_path(&local_repo_path);
    let commits_range: CommitsRange = CommitsRange::resolve_commits_range(
        &local_git_repo,
        &String::from("origin"),
        &String::from("master"),
        &String::from(""),
    );

    // should not skip the CI as we made changes on the api app
    should_skip_ci(
        &local_git_repo,
        &vec![api_app_path],
        &cmd,
        &commits_range,
    );

    // the stop command should not have been ran
    assert_eq!(false, Path::new(&witness_filepath).exists());

    // should skip the CI as we did not make any changes on the front app
    should_skip_ci(
        &local_git_repo,
        &vec![front_app_path],
        &cmd,
        &commits_range,
    );

    // the stop command should have been ran
    assert_eq!(true, Path::new(&witness_filepath).exists());

    let actual_content = read_to_string(&witness_filepath)
        .expect("Unable to read witness file content")
    ;

    assert_eq!(&actual_content, &witness_content);
}
