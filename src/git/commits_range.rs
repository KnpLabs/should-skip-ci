use git2::Repository;
use git2::Oid;
use super::branch::get_current_branch;
use super::branch::get_current_remote;
use super::branch::get_merge_base_commit;
use log::debug;

pub struct CommitsRange {
    from: Oid,
    to: Oid,
}

impl CommitsRange {
    pub fn resolve_commits_range(
        repo: &Repository,
        remote: &String,
        base_branch: &String,
        base_ref: &String,
    ) -> Self {
        let start_ref = match resolve_start_ref(
            repo,
            remote,
            base_branch,
            base_ref,
        ) {
            Ok(o) => o,
            Err(e) => panic!("Not able to resolve CommitsRange::from: {}", e),
        };

        let to = match repo.refname_to_id("HEAD") {
            Ok(o) => o,
            Err(e) => panic!("Not able to resolve CommitsRange::to: {}", e),
        };

        return Self {
            from: start_ref,
            to,
        };
    }

    pub fn from(&self) -> &Oid {
        return &self.from;
    }

    pub fn to(&self) -> &Oid {
        return &self.to;
    }
}

fn resolve_start_ref(
    repo: &Repository,
    remote: &String,
    base_branch: &String,
    base_ref: &String,
) -> Result<Oid, String> {
    let start_ref: String;

    if !base_ref.is_empty() {
        start_ref = base_ref.to_string();
        debug!("CommitsRange : Using base ref as start ref ({}).", &start_ref);

        match repo.resolve_reference_from_short_name(&start_ref) {
            Ok(r) => match r.target() {
                Some(o) => return Ok(o),
                _ => return Err(format!("Not able to resolve start ref for {}", &start_ref)),
            },
            Err(e) => return Err(e.to_string()),
        };
    }

    let start_ref_oid: Oid = match resolve_base_branch_start_ref(
        repo,
        remote,
        base_branch,
    ) {
        Ok(res) => {
            start_ref = res.to_string();
            res
        },
        Err(e) => return Err(e),
    };

    debug!("CommitsRange : Using base branch as start ref ({}).", &start_ref);

    return Ok(start_ref_oid);
}

fn resolve_base_branch_start_ref(
    repo: &Repository,
    remote: &String,
    base_branch: &String,
) -> Result<Oid, String> {
    let current_remote: String = match get_current_remote(repo) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };
    let current_branch: String = match get_current_branch(repo) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    if current_remote == remote.to_owned()
      && current_branch == base_branch.to_owned()
    {
        // When we've checked out the remote base branch, use HEAD~1
        // as the range start commit as it should be a merge commit.
        match repo.revparse_single("HEAD~1") {
            Ok(obj) => return Ok(obj.id()),
            Err(e) => return Err(e.to_string())
        };
    }

    // When we're not on the base branch, we determine the range start commit
    // from the point when the current branch has been issued from the base
    // branch.
    return get_merge_base_commit(
        repo,
        remote,
        base_branch,
    );
}
