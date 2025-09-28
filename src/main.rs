pub mod issues;

use issues::{create_issue, fetch_issues, Error, Issue};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let issues = fetch_issues().await?;

    println!("Found issues {}", issues.len());
    for issue in issues.iter() {
        println!("{:?}", issue);
    }

    let new_issue = Issue {
        msg: "TODO: Fix this".to_string(),
        assignee: "sulfastor".to_string(),
        number: None,
        status: None,
    };

    create_issue(new_issue).await?;

    Ok(())
}
