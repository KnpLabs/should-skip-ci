use std::path::PathBuf;
use std::process::Command;
use std::fs::{
    File,
    write,
    OpenOptions,
};
// import std::io::Write in order to use the writeln! macro on a std::fs::File
use std::io::Write;
use git2::Repository;

pub fn open_repository_at_path(path: &PathBuf) -> Repository
{
    match Repository::open(&path) {
        Ok(repo) => return repo,
        Err(e) => panic!("Failed to open repository {}", e),
    };
}

pub fn create_remote_repo(
    base_path: &PathBuf,
) -> PathBuf {
    let mut repo_path = PathBuf::from(base_path);

    repo_path.push("remote");

    Command::new("mkdir")
        .arg("-p")
        .arg(&repo_path)
        .output() // synchrone execution
        .expect(&format!(
            "Unable to create remote dir repo dir {}",
            &repo_path.to_str().unwrap(),
        ))
    ;

    Command::new("git")
        .arg("init")
        .current_dir(&repo_path)
        .output() // synchrone execution
        .expect(&format!(
            "Unable to initialize remote git repo in {}",
            &repo_path.to_str().unwrap(),
        ))
    ;

    // Set the checked out branch to a name that is only used on the remote
    // repo directory, in order to be able to push to `master`.
    // Otherwise git will complain that `master` is checked out on the remote
    // path, and so we can't push to it.
    // @see git receive.denyCurrentBranch option
    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg("private-remote-branch")
        .current_dir(&repo_path)
        .output() // synchrone execution
        .expect(&format!(
            "Unable to checkout the private branch for the remote repo dir {}",
            &repo_path.to_str().unwrap(),
        ))
    ;

    return repo_path;
}

pub fn create_local_repo(
    base_path: &PathBuf,
) -> PathBuf {
    let mut repo_path = PathBuf::from(base_path);

    repo_path.push("local");

    Command::new("mkdir")
        .arg("-p")
        .arg(&repo_path)
        .output() // synchrone execution
        .expect(&format!(
            "Unable to create local dir repo dir {}",
            &repo_path.to_str().unwrap(),
        ))
    ;

    Command::new("git")
        .arg("init")
        .current_dir(&repo_path)
        .output() // synchrone execution
        .expect(&format!(
            "Unable to initialize local git repo in {}",
            &repo_path.to_str().unwrap(),
        ))
    ;

    Command::new("git")
        .arg("config")
        .arg("user.email")
        .arg("ssc-test@acme.com")
        .current_dir(&repo_path)
        .output()
        .expect("Unable to set user.email git config")
    ;

    Command::new("git")
        .arg("config")
        .arg("user.name")
        .arg("ssc-test")
        .current_dir(&repo_path)
        .output()
        .expect("Unable to set user.name git config")
    ;

    return repo_path;
}

pub fn set_remote_repo(
    local_repo_path: &PathBuf,
    remote_repo_path: &PathBuf,
) {
    Command::new("git")
        .arg("remote")
        .arg("add")
        .arg("origin")
        .arg(&remote_repo_path)
        .current_dir(&local_repo_path)
        .output()
        .expect(&format!(
            "Unable to set git remote {} for local repo {}",
            &remote_repo_path.to_str().unwrap(),
            &local_repo_path.to_str().unwrap(),
        ))
    ;
}

pub fn create_and_push_initial_commit(
    repo_path: &PathBuf,
    branch_name: &String,
) {
    let filename = "README.md";
    let apps = vec!["apps/api", "apps/front"];

    for app_path in &apps {
        Command::new("mkdir")
            .arg("-p")
            .arg(&app_path)
            .current_dir(&repo_path)
            .output()
            .expect(&format!(
                "Unable to create app path {}",
                &app_path,
            ))
        ;

        let mut filepath = PathBuf::from(&repo_path);
        filepath.push(&app_path);
        filepath.push(&filename);

        write(
            &filepath,
            "# Test file",
        ).expect(&format!(
            "Unable to write {} file",
            &filepath.to_str().unwrap(),
        ));
    }

    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(&repo_path)
        .output()
        .expect(&format!(
            "Unable to add {} to git staging area.",
            &repo_path.to_str().unwrap(),
        ))
    ;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg("initial commit")
        .current_dir(&repo_path)
        .output()
        .expect("Unable to perform initial commit.")
    ;

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(&branch_name)
        .current_dir(&repo_path)
        .output()
        .expect(&format!(
            "Unable to push {} branch to remote.",
            &branch_name,
        ))
    ;
}

pub fn create_and_push_tag(
    repo_path: &PathBuf,
    tag_name: &String,
) {
    Command::new("git")
        .arg("tag")
        .arg("-a")
        .arg(&tag_name)
        .arg("-m")
        .arg(&format!(
            "test tag {}",
            &tag_name,
        ))
        .current_dir(&repo_path)
        .output()
        .expect(&format!(
            "Unable to create tag {} for repo {}",
            &tag_name,
            &repo_path.to_str().unwrap(),
        ))
    ;

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(&tag_name)
        .current_dir(&repo_path)
        .output()
        .expect(&format!(
            "Unable to push tag {} for repo {}",
            &tag_name,
            &repo_path.to_str().unwrap(),
        ))
    ;
}

pub fn checkout_new_branch(
    repo_path: &PathBuf,
    branch_name: &String,
) {
    Command::new("git")
        .arg("checkout")
        .arg("-b")
        .arg(&branch_name)
        .current_dir(&repo_path)
        .output()
        .expect(&format!(
            "Unable to checkout new branch {} for repo {}",
            &branch_name,
            &repo_path.to_str().unwrap(),
        ))
    ;
}

pub fn append_content_to_api_readme(
    repo_path: &PathBuf,
    content: &String,
) {
    let mut filepath = PathBuf::from(&repo_path);
    filepath.push("apps/api");
    filepath.push("README.md");

    let mut file: File = OpenOptions::new()
        .append(true)
        .open(&filepath)
        .unwrap()
    ;

    writeln!(file, "{}", &content).unwrap();
}

pub fn commit_and_push_changes(
    repo_path: &PathBuf,
    branch_name: &String,
    message: &String,
) {
    Command::new("git")
        .arg("add")
        .arg(".")
        .current_dir(&repo_path)
        .output()
        .expect("Unable to add changes to git staging area.")
    ;

    Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(&message)
        .current_dir(&repo_path)
        .output()
        .expect("Unable to commit changes")
    ;

    Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(&branch_name)
        .current_dir(&repo_path)
        .output()
        .expect("Unable to push changes")
    ;
}

pub fn create_api_test_file(
    repo_path: &PathBuf,
    file_name: &String,
    content: &String,
) {
    let mut filepath = PathBuf::from(&repo_path);
    filepath.push("apps/api");
    filepath.push(&file_name);

    write(
        &filepath,
        &content,
    ).expect(&format!(
        "Unable to write {} file",
        &filepath.to_str().unwrap(),
    ));
}

pub fn checkout_branch(
    repo_path: &PathBuf,
    branch_name: &String,
) {
    Command::new("git")
        .arg("checkout")
        .arg(&branch_name)
        .current_dir(&repo_path)
        .output()
        .expect(&format!(
            "Unable to go on the {} branch.",
            &branch_name,
        ))
    ;
}

pub fn merge_given_branch_on_current_branch_non_fast_forward(
    repo_path: &PathBuf,
    branch_name: &String,
    message: &String,
) {
    Command::new("git")
        .arg("merge")
        .arg("--no-ff")
        .arg(&branch_name)
        .arg("-m")
        .arg(&message)
        .current_dir(&repo_path)
        .output()
        .expect("Unable to merge the branch into the base branch")
    ;
}
