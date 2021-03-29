use lazy_static::lazy_static;
use regex::Regex;
use reqwest::StatusCode;
use serde::Serialize;
use std::process::exit;
use structopt::StructOpt;
use tokio::task::{JoinError, JoinHandle};

/// A command line tool to check if GitHub PR URLs are merged or not.
#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Opt {
    /// GitHub pull request URLs
    #[structopt(short, long)]
    urls: Vec<String>,
}

lazy_static! {
    static ref GH_PR: Regex = Regex::new(r"^https://github\.com/(.*?)/(.*?)/pull/(.*?)$").unwrap();
}

#[derive(Serialize)]
struct Report {
    url: String,
    has_valid_format: bool,
    is_merged: bool,
    has_error: bool,
}

impl Report {
    fn new(url: String) -> Self {
        Report {
            url,
            has_valid_format: true,
            is_merged: false,
            has_error: false,
        }
    }
}

#[derive(Serialize)]
struct Output {
    reports: Vec<Report>,
    failed_qty: u32,
}

// https://docs.github.com/en/rest/reference/pulls#check-if-a-pull-request-has-been-merged
// https://github.com/Restfulness/Restfulness-flutter-app/pull/36
// https://api.github.com/repos/octocat/hello-world/pulls/42/merge

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();
    let output = generate_output(opt).await?;

    println!("{}", serde_json::to_string(&output).unwrap());

    exit(output.failed_qty as i32);
}

async fn generate_output(opt: Opt) -> Result<Output, JoinError> {
    let mut output = Output {
        reports: Vec::new(),
        failed_qty: 0,
    };
    let handles = opt
        .urls
        .iter()
        .map(|url| github_pull_merge_checker(url.clone()))
        .collect::<Vec<_>>();

    for h in handles {
        let report = h.await?;
        if !report.is_merged {
            output.failed_qty += 1;
        }
        output.reports.push(report);
    }

    Ok(output)
}

fn github_pull_merge_checker(url: String) -> JoinHandle<Report> {
    tokio::spawn(async move {
        let mut report = Report::new(url);

        let caps = match GH_PR.captures(&report.url) {
            None => {
                report.has_valid_format = false;
                return report;
            }
            Some(res) => res,
        };

        let name = caps.get(1).map_or("", |m| m.as_str());
        let repo = caps.get(2).map_or("", |m| m.as_str());
        let pull = caps.get(3).map_or("", |m| m.as_str());

        let url = format!(
            "https://api.github.com/repos/{}/{}/pulls/{}/merge",
            name, repo, pull
        );

        let client = match reqwest::Client::builder()
            .user_agent("Farid the agent")
            .build()
        {
            Ok(c) => c,
            Err(_) => {
                report.has_error = true;
                return report;
            }
        };

        let status = match client.get(&url).send().await {
            Ok(r) => r.status(),
            Err(_) => {
                report.has_error = true;
                return report;
            }
        };

        match status {
            StatusCode::NOT_FOUND => report.is_merged = false,
            StatusCode::NO_CONTENT => report.is_merged = true,
            _ => report.has_error = true,
        };

        report
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_add() {
        let opt = Opt {
            urls: vec![
                "https://github.com/Restfulness/Restfulness-flutter-app/pull/36".to_string(),
                "https://github.com/Restfulness/Restfulness-flutter-app/pull/35".to_string(),
            ],
        };
        let output = generate_output(opt).await.unwrap();

        assert_eq!(output.failed_qty, 1);
        assert_eq!(output.reports.len(), 2);
    }
}
