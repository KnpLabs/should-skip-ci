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
    create_remote_repo,
    create_local_repo,
    set_remote_repo,
    create_and_push_initial_commit,
    create_and_push_tag,
    append_content_to_api_readme,
    commit_and_push_changes,
};

#[test]
fn it_should_detect_changes_from_a_tag() {
    // Setup test case

    let rdm = rand::random::<u32>();
    let base_path = PathBuf::from(format!(
        "/tmp/ssc-functional-test-{}",
        rdm,
    ));
    let remote_repo_path = create_remote_repo(&base_path);
    let local_repo_path = create_local_repo(&base_path);
    let branch_name = String::from("master");
    let tag_name = String::from("v0.0.1");

    set_remote_repo(&local_repo_path, &remote_repo_path);

    create_and_push_initial_commit(
        &local_repo_path,
        &branch_name,
    );

    create_and_push_tag(
        &local_repo_path,
        &tag_name,
    );

    append_content_to_api_readme(
        &local_repo_path,
        &String::from("\nmore content"),
    );

    commit_and_push_changes(
        &local_repo_path,
        &branch_name,
        &String::from("first commit since tag"),
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
        &String::from(""),
        &String::from("v0.0.1"),
    );

    // should not skip the CI as we made changes on the api app since the tag
    should_skip_ci(
        &local_repo_path,
        &vec![api_app_path],
        &cmd,
        &commits_range,
    );

    // the stop command should not have been ran
    assert_eq!(false, Path::new(&witness_filepath).exists());

    // should skip the CI as we did not make any changes on the front app
    // since the tag
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
