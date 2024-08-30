use std::path::PathBuf;
use super::branch::get_current_branch;
use super::branch::get_current_remote;
use super::branch::get_merge_base_commit;
use log::debug;

pub struct CommitsRange {
    from: String,
    to: String,
}

impl CommitsRange {
    pub fn resolve_commits_range(
        working_directory: &PathBuf,
        remote: &String,
        base_branch: &String,
        base_ref: &String,
    ) -> Self {
        let start_ref: String = resolve_start_ref(
            working_directory,
            remote,
            base_branch,
            base_ref,
        );

        return CommitsRange {
            from: start_ref,
            to: String::from("HEAD"),
        };
    }
    pub fn from(&self) -> &String {
        return &self.from;
    }

    pub fn to(&self) -> &String {
        return &self.to;
    }
}

fn resolve_start_ref(
    working_directory: &PathBuf,
    remote: &String,
    base_branch: &String,
    base_ref: &String,
) -> String {
    let start_ref: String;

    if !base_ref.is_empty() {
        start_ref = base_ref.to_string();
        debug!("CommitsRange : Using base ref as start ref ({}).", &start_ref);

        return start_ref;
    }

    start_ref = resolve_base_branch_start_ref(
        &working_directory,
        remote,
        base_branch,
    );

    debug!("CommitsRange : Using base branch as start ref ({}).", &start_ref);

    return start_ref;
}

fn resolve_base_branch_start_ref(
    working_directory: &PathBuf,
    remote: &String,
    base_branch: &String,
) -> String {
    let current_remote: String = get_current_remote(&working_directory);
    let current_branch: String = get_current_branch(&working_directory);

    if current_remote == remote.to_owned()
      && current_branch == base_branch.to_owned()
    {
        // When we've checked out the remote base branch, use HEAD~1
        // as the range start commit as it should be a merge commit.
        return String::from("HEAD~1");
    }

    // When we're not on the base branch, we determine the range start commit
    // from the point when the current branch has been issued from the base
    // branch.
    return get_merge_base_commit(
        &working_directory,
        remote,
        base_branch,
    );
}
