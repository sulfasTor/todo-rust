use git2::Repository;
use octocrab::models::IssueState;
use octocrab::{models, params, Error};

#[derive(Debug)]
struct RepoConfig {
    Username: String,
    RepoName: String,
}

#[derive(Debug)]
pub struct Issue {
    Number: u64,
    Message: String,
    Status: IssueState,
}

fn getRepoConfig() -> Option<RepoConfig> {
    let repo = Repository::open(".").ok()?;
    let config = repo.config().ok()?;
    let origin_url = config
        .get_string("remote.origin.url")
        .map_err(|e| git2::Error::from_str(&format!("Failed to get remote.origin.url: {}", e)))
        .ok()?;

    let parts: Vec<&str> = origin_url.split("/").collect();
    let username = parts[3];
    let repo = parts[4].replace(".git", "");

    Some(RepoConfig {
        Username: username.to_string(),
        RepoName: repo.to_string(),
    })
}

pub async fn fetch_issues() -> Result<Vec<Issue>, Error> {
    let rc = getRepoConfig().unwrap();
    let octocrab = octocrab::instance();
    let page = octocrab
        .issues(rc.Username, rc.RepoName)
        .list()
        // Optional Parameters
        .state(params::State::Open)
        .per_page(50)
        .send()
        .await?;

    // Go through every page of issues. Warning: There's no rate limiting so
    // be careful.
    let results = octocrab.all_pages::<models::issues::Issue>(page).await?;
    let issues = results
        .iter()
        .map(|s| Issue {
            Number: s.number,
            Status: IssueState::Open,
            Message: s.title.to_string(),
        })
        .collect();

    Ok(issues)
}
