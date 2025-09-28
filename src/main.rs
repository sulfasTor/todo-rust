pub mod issues;
pub mod parser;

use issues::{create_issues, fetch_issues, Error, Issue};
use parser::{parse_todos, Todo};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let issues = fetch_issues().await?;

    println!("Found issues {}", issues.len());
    for issue in issues.iter() {
        println!("{:?}", issue);
    }

    let todos = parse_todos();

    println!("Found todos {}", todos.len());
    for todo in &todos {
        println!("{:?}", todo)
    }

    let issues: Vec<Issue> = todos.into_iter().map(|t| t.into_issue()).collect();
    create_issues(issues).await?;

    Ok(())
}
