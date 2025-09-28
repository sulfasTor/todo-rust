use git2::Repository;
use octocrab::models::IssueState;
use octocrab::{models, params, Error, Octocrab, Result};

#[derive(Debug)]
struct RepoConfig {
    user_name: String,
    repo_name: String,
}

#[derive(Debug)]
pub struct Issue {
    pub number: Option<u64>,
    pub msg: String,
    pub status: Option<IssueState>,
    pub assignee: String,
}

fn get_repo_config() -> Option<RepoConfig> {
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
        user_name: username.to_string(),
        repo_name: repo.to_string(),
    })
}

pub async fn fetch_issues() -> Result<Vec<Issue>, Error> {
    let rc = get_repo_config().unwrap();
    let octocrab = octocrab::instance();
    let page = octocrab
        .issues(rc.user_name, rc.repo_name)
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
            number: Some(s.number),
            status: Some(IssueState::Open),
            msg: s.title.to_string(),
            assignee: s.assignee.as_ref().unwrap().login.to_string(),
        })
        .collect();

    Ok(issues)
}

pub async fn create_issues(issues: Vec<Issue>) -> Result<(), Error> {
    let rc = get_repo_config().unwrap();

    let token = std::env::var("GH_TOKEN").expect("GH_TOKEN env variable is required");
    let octocrab = Octocrab::builder().personal_token(token).build()?;

    for issue in issues.iter() {
        octocrab
            .issues(rc.user_name.clone(), rc.repo_name.clone())
            .create(issue.msg.clone())
            .assignees(vec![issue.assignee.clone()])
            .body(issue.msg.clone())
            .send()
            .await?;
    }
    Ok(())
}
