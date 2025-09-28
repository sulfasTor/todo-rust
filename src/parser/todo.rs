use crate::Issue;
use regex::Regex;
use std::ffi::OsStr;
use std::fs::{read_dir, read_to_string, File};
use std::io::{self, BufRead};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

#[derive(Debug)]
pub struct Todo {
    file: PathBuf,
    msg: String,
    assignee: Option<String>,
    line: usize,
}

pub type Todos = Vec<Todo>;

impl Todo {
    pub fn into_issue(self) -> Issue {
        let file_str = self.file.display().to_string();
        let formatted_msg = format!("TODO: {} at {} in line {}", self.msg, file_str, self.line);

        Issue {
            number: None,
            title: self.msg,
            body: formatted_msg,
            status: None,
            assignee: self.assignee,
        }
    }
}

fn is_code_file(extension: Option<&OsStr>) -> bool {
    // TODO(sulfastor): Add more programming languagues comment agnostic
    const CODE_EXTENSIONS: &[&str] = &["rs", "md", "go"];

    extension
        .and_then(|ext| ext.to_str())
        .map(|ext| CODE_EXTENSIONS.contains(&ext))
        .unwrap_or(false)
}

fn glob_valid_files() -> Vec<PathBuf> {
    WalkDir::new("./")
        .into_iter()
        .filter_map(|entry| {
            let path = entry.ok()?.path().to_path_buf();
            if path.is_file() && is_code_file(path.extension()) {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if !name.starts_with('.') {
                        return Some(path);
                    }
                }
            }
            None
        })
        .collect()
}

pub fn parse_todos() -> Todos {
    let files = glob_valid_files();
    let re =
        Regex::new(r"(?m)^\s*//\s*TODO(?:\((?P<assignee>[^)]+)\))?:\s*(?P<msg>.+)\s*$").unwrap();

    let mut todos = Vec::new();

    for f in files {
        let content = match read_to_string(&f) {
            Ok(c) => c,
            Err(_) => continue,
        };

        for (i, line) in content.lines().enumerate() {
            if let Some(caps) = re.captures(line) {
                let assignee = caps.name("assignee").map(|m| m.as_str().to_string());
                let msg = caps["msg"].trim().to_string();

                todos.push(Todo {
                    file: f.clone(),
                    line: i + 1,
                    assignee,
                    msg,
                });
            }
        }
    }

    todos
}
