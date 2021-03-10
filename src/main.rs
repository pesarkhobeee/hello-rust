use regex::Regex;
use reqwest::StatusCode;
use std::process::exit;
use structopt::StructOpt;

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "basic")]
struct Opt {
    /// Github pull URLs
    #[structopt(short, long)]
    urls: Vec<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();
    for url in opt.urls {
        github_pull_merge_checker(&url).await?;
    }
    // https://docs.github.com/en/rest/reference/pulls#check-if-a-pull-request-has-been-merged
    // https://github.com/Restfulness/Restfulness-flutter-app/pull/36
    // https://api.github.com/repos/octocat/hello-world/pulls/42/merge
    Ok(())
}

async fn github_pull_merge_checker(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let re = Regex::new(r"^https://github\.com/(.*?)/(.*?)/pull/(.*?)$").unwrap();

    let caps = match re.captures(&url) {
        None => {
            println!("{}", url);
            exit(2)
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

    let client = reqwest::Client::builder()
        .user_agent("Farid the agent")
        .build()?;

    let status = client.get(&url).send().await?.status();

    println!("{}", url);

    match status {
        StatusCode::NOT_FOUND => {
            println!("Not Merged");
            exit(1)
        }
        StatusCode::NO_CONTENT => println!("Merged"),
        _ => println!("{}", url),
    }

    Ok(())
}
