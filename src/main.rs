use std::{
    fs::{File, OpenOptions},
    io::Write,
    process::exit,
};

mod inputs;

mod results;

mod security;
use github_actions::issue_command;
use security::*;
use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let inputs = inputs::gather_inputs();

    if inputs.is_none() {
        github_actions::error!("Invalid or missing inputs.");
        exit(1);
    }

    let mut set = JoinSet::new();

    let input_result = inputs.unwrap();

    set.spawn(verify_dependabot_yaml(input_result.clone()));
    set.spawn(verify_dependabot_enabled(input_result.clone()));

    let mut failed = false;

    let results = set.join_all().await;

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(input_result.step_summary_path)
        .unwrap();

    file.write_all("## Repository Compliance Report\n\n".as_bytes())
        .unwrap();

    for result in results {
        file.write_all(result.into_markdown().as_bytes()).unwrap();
        match result {
            results::CheckResult::Failure(_) => {
                failed = true;
            }
            _ => (),
        }
    }

    if failed {
        file.write_all("\n**RESULT: FAILURE**\n".as_bytes())
            .unwrap();
        file.flush().unwrap();
        exit(1);
    }
    file.write_all("\n**RESULT: SUCCESS**\n".as_bytes())
        .unwrap();
    file.flush().unwrap();
}
