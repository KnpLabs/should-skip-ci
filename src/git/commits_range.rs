use crate::cli::Cli;
use crate::git::branch::get_current_branch;
use crate::git::branch::get_merge_base_commit;

pub struct CommitsRange {
    from: String,
    to: String,
}

impl CommitsRange {
    pub fn to_str(&self) -> String {
        return format!(
            "{}..{}",
            &self.from,
            &self.to,
        );
    }
}

pub fn resolve_commits_range(cli: &Cli) -> CommitsRange {
    return CommitsRange{
        from: resolve_range_start_commit(cli.base_branch()),
        to: String::from("HEAD"),
    }
}

fn resolve_range_start_commit(base_branch: &String) -> String {
    let current_branch: String = get_current_branch();

    if current_branch == base_branch.to_owned() {
        // When we're on the base branch, use HEAD~1 as the range start commit
        // as it should be a merge commit.
        return String::from("HEAD~1");
    }

    // When we're not on the base branch, we determine the range start commit
    // from the point when the current branch has been issued from the base
    // branch.
    return get_merge_base_commit(base_branch);
}
