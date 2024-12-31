use git2::Repository;
use git2::Branch;
use git2::BranchType;
use git2::Oid;

pub fn get_current_branch(
    repo: &Repository,
) -> Result<String, String> {
    let branches = match repo.branches(Some(BranchType::Local)) {
        Ok(branches) => branches,
        Err(e) => return Err(format!("Failed to get branches {}", e)),
    };

    for row in branches {
        let branch: Branch = match row {
            Ok(b) => b.0,
            _ => continue,
        };

        if !branch.is_head() {
            continue;
        }

        let branch_name = match branch.name() {
            Ok(res) => match res {
                Some(n) => n,
                None => return Err(String::from("Unable to fetch current branch name")),
            },
            _ => return Err(String::from("Unable to fetch current branch name")),
        };

        return Ok(String::from(branch_name));
    }

    return Err(format!("Not able to get current branch for repo {:?}", repo.path()));
}

/// Returns the current (i.e. the first) remote for the given repo.
///
/// See `$ git remote show`.
///
/// Returns the first line of the above command.
pub fn get_current_remote(
    repo: &Repository,
) -> Result<String, String> {
    let remotes = match repo.remotes() {
        Ok(res) => res,
        Err(e) => return Err(format!("Not able to fetch remotes for repo {:?}, {}", repo.path(), e)),
    };

    match remotes.get(0) {
        Some(name) => return Ok(String::from(name)),
        _ => return Err(format!("No remotes for repo {:?}", repo.path())),
    };
}

pub fn get_merge_base_commit(
    repo: &Repository,
    remote: &String,
    base_branch: &String,
) -> Result<Oid, String> {
    let from_ref = format!("{}/{}", remote, base_branch);

    let from = match repo.resolve_reference_from_short_name(&from_ref) {
        Ok(r) => match r.target() {
            Some(o) => o,
            _ => return Err(format!("Not able to get 'from' merge base commit ({}).", &from_ref)),
        },
        Err(e) => return Err(format!("Not able to get 'from' merge base commit ({}): {}", &from_ref, e)),
    };
    let to = match repo.refname_to_id("HEAD") {
        Ok(o) => o,
        Err(e) => return Err(format!("Not able to get to merge base commit: {}", e)),
    };

    match repo.merge_base(from, to) {
        Ok(mb) => return Ok(mb),
        Err(e) => return Err(format!("Error while getting merge base commit: {}", e)),
    };
}
