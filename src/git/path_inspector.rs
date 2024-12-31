extern crate log;

use std::path::PathBuf;
use git2::Repository;
use git2::DiffOptions;
// use log::{debug, info};

use super::commits_range::CommitsRange;

pub fn has_changes_in_paths(
    repo: &Repository,
    commits_range: &CommitsRange,
    paths: &Vec<PathBuf>,
) -> bool {
    let from_tree = match repo.revparse_single(format!("{}", commits_range.from()).as_str()) {
        Ok(o) => match o.peel_to_tree() {
            Ok(t) => t,
            _ => panic!("Tree not found for from commit range ({})", commits_range.from()),
        },
        _ => panic!("Tree not found for from commit range ({})", commits_range.from()),
    };

    let to_tree = match repo.revparse_single(format!("{}", commits_range.to()).as_str()) {
        Ok(o) => match o.peel_to_tree() {
            Ok(t) => t,
            _ => panic!("Tree not found for to commit range ({})", commits_range.to()),
        },
        _ => panic!("Tree not found for to commit range ({})", commits_range.to()),
    };

    let repo_path = &repo.path().parent().unwrap();
    let mut diff_opts: DiffOptions = DiffOptions::new();
    for path in paths {
        match path.strip_prefix(repo_path) {
            Ok(p) => diff_opts.pathspec(&p),
            _ => continue,
        };
    }

    let diffs = match repo.diff_tree_to_tree(Some(&from_tree), Some(&to_tree), Some(&mut diff_opts)) {
        Ok(d) => d,
        Err(_) => return false,
    };

    let deltas = diffs.deltas();
    let deltas_count = deltas.len();
    /*
    info!(
        "Detected diff from {} to {} :\n{}",
        commits_range.from(),
        commits_range.to(),
        String::from_utf8(result.stdout.to_vec()).unwrap()
    );
    */

    return deltas_count > 0;
}
