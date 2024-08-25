mod utils;

use std::path::{
    Path,
    PathBuf,
};
use std::fs::read_to_string;
use rand;
use ssc::should_skip_ci;
use ssc::git::commits_range::CommitsRange;
use utils::{
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

/*

This test file is meant to test a `master` and `develop` branches git based
workflow.
In this workflow, the `master` branch is ISO to the prod, and the `develop`
branch contains the latest features.
Developers usually issue new feature branches from the `develop` branch.
The workflow is represented by the following schema :

master ---o-------o
           \     /
develop     o---o
             \ /
feature1      o


---> direction

*/

#[test]
fn it_should_detect_changes_on_master_when_merging_develop_on_master_and_having_develop_as_the_base_branch() {
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

    let develop_branch_name = String::from("develop");
    checkout_new_branch(&local_repo_path, &develop_branch_name);

    append_content_to_api_readme(
        &local_repo_path,
        &String::from("\nmore content"),
    );

    commit_and_push_changes(
        &local_repo_path,
        &develop_branch_name,
        &String::from("first commit on develop"),
    );

    let feature_branch_name = String::from("feature/first-one");
    checkout_new_branch(&local_repo_path, &feature_branch_name);

    create_api_test_file(
        &local_repo_path,
        &String::from("test.txt"),
        &String::from("test content"),
    );

    commit_and_push_changes(
        &local_repo_path,
        &feature_branch_name,
        &String::from("first commit on feature branch"),
    );

    checkout_branch(
        &local_repo_path,
        &develop_branch_name,
    );

    merge_given_branch_on_current_branch_non_fast_forward(
        &local_repo_path,
        &feature_branch_name,
        &String::from("merge feature branch into develop"),
    );

    checkout_branch(
        &local_repo_path,
        &String::from("master"),
    );

    merge_given_branch_on_current_branch_non_fast_forward(
        &local_repo_path,
        &develop_branch_name,
        &String::from("merge develop branch into master"),
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

    let commits_range: CommitsRange = CommitsRange::resolve_commits_range(
        &local_repo_path,
        &String::from("origin"),
        &develop_branch_name,
        &String::from(""),
    );

    // should not skip the CI as we made changes on the api app
    should_skip_ci(
        &local_repo_path,
        &vec![api_app_path],
        &cmd,
        &commits_range,
    );

    // the stop command should not have been ran
    assert_eq!(false, Path::new(&witness_filepath).exists());

    // should skip the CI as we did not make any changes on the front app
    should_skip_ci(
        &local_repo_path,
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
