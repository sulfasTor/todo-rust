pub mod issues;

use issues::{fetch_issues, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let issues = fetch_issues().await?;

    println!("Found issues {}", issues.len());
    for issue in issues.iter() {
        println!("{:?}", issue);
    }

    Ok(())
}
