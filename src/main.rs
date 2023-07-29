use std::process::exit;

use reqwest::StatusCode;
use structopt::StructOpt;
use tokio::task::{JoinError, JoinHandle};

use crate::url_utils::get_api_url;

mod http_request;
mod models;
mod url_utils;

#[cfg(test)]
mod test;

/// A command line tool to check if GitHub PR URLs are merged or not.
#[derive(StructOpt)]
#[structopt(name = "basic")]
struct Opt {
    /// GitHub pull request URLs
    #[structopt(short, long)]
    urls: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt: Opt = Opt::from_args();
    let output = generate_output(opt).await?;

    match serde_json::to_string(&output) {
        Ok(s) => println!("{}", s),
        Err(_) => exit(126),
    }

    exit(output.failed_qty as i32);
}

async fn generate_output(opt: Opt) -> Result<models::Output, JoinError> {
    let mut output = models::Output {
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

fn github_pull_merge_checker(url: String) -> JoinHandle<models::Report> {
    tokio::spawn(async move {
        let mut report = models::Report::new(url.clone());

        let (name, repo, pull) = match url_utils::extract_params(&url) {
            Err(_) => {
                report.has_error = true;
                return report;
            }
            Ok((n, r, p)) => (n, r, p),
        };

        let status = match http_request::get_status(&get_api_url(name, repo, pull)).await {
            Ok(code) => code,
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
