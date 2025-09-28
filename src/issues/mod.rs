pub mod issues;

pub use issues::{create_issue, fetch_issues, Issue};
pub use octocrab::Error;
