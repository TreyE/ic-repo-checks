use std::process::exit;

mod inputs;

mod check_result;

mod security;
use github_actions::{issue_command, GITHUB_REPOSITORY, GITHUB_REPOSITORY_OWNER, GITHUB_SHA};
use security::*;
use tokio::task::JoinSet;

static GITHUB_TOKEN: &str = "GITHUB_TOKEN";
static INPUT_ACCESS_TOKEN: &str = "INPUT_ACCESS_TOKEN";

fn read_input(input_name: &str) -> Result<String, String> {
    let r_val = std::env::var(input_name);
    match r_val {
        Err(y) => match y {
            std::env::VarError::NotPresent => Err(format!("{} was not provided.", input_name)),
            std::env::VarError::NotUnicode(_z) => {
                Err(format!("{} was not properly encoded.", input_name))
            }
        },
        Ok(x) => Ok(x),
    }
}

fn gather_inputs() -> Option<inputs::Inputs> {
    let r_owner = read_input(GITHUB_REPOSITORY_OWNER);

    let mut failed = false;

    if r_owner.is_err() {
        github_actions::error!(r_owner.as_ref().expect_err("should be an error"));
        failed = true;
    }
    let r_repo = read_input(GITHUB_REPOSITORY);
    if r_repo.is_err() {
        github_actions::error!(r_repo.as_ref().expect_err("should be an error"));
        failed = true;
    }
    let r_sha = read_input(GITHUB_SHA);
    if r_sha.is_err() {
        github_actions::error!(r_sha.as_ref().expect_err("should be an error"));
        failed = true;
    }
    let r_token = read_input(GITHUB_TOKEN);
    if r_token.is_err() {
        github_actions::error!(r_token.as_ref().expect_err("should be an error"));
        failed = true;
    }

    let a_token = read_input(INPUT_ACCESS_TOKEN);
    if a_token.is_err() {
        github_actions::error!(a_token.as_ref().expect_err("should be an error"));
        failed = true;
    }

    if failed {
        return None;
    }

    Some(inputs::Inputs {
        repository_owner: r_owner.unwrap(),
        repository: r_repo.unwrap(),
        token: r_token.unwrap(),
        sha: r_sha.unwrap(),
        access_token: a_token.unwrap(),
    })
}

#[tokio::main]
async fn main() {
    let inputs = gather_inputs();

    if inputs.is_none() {
        github_actions::error!("Invalid or missing inputs.");
        exit(1);
    }

    let mut set = JoinSet::new();

    let input_result = inputs.unwrap();

    set.spawn(verify_dependabot_yaml(input_result.clone()));
    set.spawn(verify_dependabot_enabled(input_result.clone()));

    let mut failed = false;

    for result in set.join_all().await {
        match result {
            check_result::CheckResult::Failure(x) => {
                github_actions::error!(format!("{}", x));
                failed = true;
            }
            check_result::CheckResult::Pass => (),
        }
    }

    if failed {
        github_actions::error!("Failed repository policy checks");
        exit(1);
    }
}
