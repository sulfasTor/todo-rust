pub mod issues;
pub mod parser;

use issues::{create_issues, fetch_issues, Error, Issue};
use octocrab::models::IssueState;
use parser::{parse_todos, Todo};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let issues = fetch_issues().await?;
    println!("Found issues {}", issues.len());
    for issue in issues.iter() {
        println!("{:?}", issue);
    }

    let mut open_keys: HashSet<String> = issues
        .iter()
        .filter(|i| matches!(i.status, Some(IssueState::Open)))
        .map(|i| normalize_key(&i.body))
        .collect();

    let todos = parse_todos();
    println!("Found todos {}", todos.len());
    for todo in &todos {
        println!("{:?}", todo)
    }

    let mut batch_seen: HashSet<String> = HashSet::new();
    let new_issues: Vec<Issue> = todos
        .into_iter()
        .map(|t| t.into_issue())
        .filter(|iss| {
            let key = normalize_key(&iss.body);
            !open_keys.contains(&key) && batch_seen.insert(key)
        })
        .collect();

    if new_issues.is_empty() {
        println!("No new issues to create.");
        return Ok(());
    }

    create_issues(new_issues).await?;
    Ok(())
}

fn normalize_key(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .to_lowercase()
}
