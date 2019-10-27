use crate::cli::Cli;

pub struct CommitsRange {
    from: String,
    to: String,
}

impl CommitsRange {
    pub fn to_str(&self) -> String {
        return format!(
            "{}^..{}",
            &self.from,
            &self.to,
        );
    }
}

pub fn resolve_commits_range(cli: &Cli) -> CommitsRange {
    // @TODO consider the --pr-url flag to determine the commits range
    return CommitsRange{
        from: String::from(cli.from()),
        to: String::from(cli.to()),
    }
}
