use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref GH_PR: Regex = Regex::new(r"^https://github\.com/(.*?)/(.*?)/pull/(.*?)$").unwrap();
}

// github api docs: https://docs.github.com/en/rest/reference/pulls#check-if-a-pull-request-has-been-merged

// input url: https://github.com/Restfulness/Restfulness-flutter-app/pull/36
pub(crate) fn extract_params(url: &str) -> Result<(&str, &str, &str), &str> {
    match GH_PR.captures(url) {
        None => Err("Not Matched"),

        Some(caps) => {
            let name = caps.get(1).map_or("", |m| m.as_str());
            let repo = caps.get(2).map_or("", |m| m.as_str());
            let pull = caps.get(3).map_or("", |m| m.as_str());

            Ok((name, repo, pull))
        }
    }
}

// api url: https://api.github.com/repos/octocat/hello-world/pulls/42/merge
pub(crate) fn get_api_url(name: &str, repo: &str, pull: &str) -> String {
    format!(
        "https://api.github.com/repos/{}/{}/pulls/{}/merge",
        name, repo, pull
    )
}
