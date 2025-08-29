use std::{error::Error, fs::OpenOptions, io::Write, process::exit};

mod copilot;

mod id_tree;

mod inputs;

mod results;

mod quality;

mod dependabot;
use dependabot::*;
use github_actions::issue_command;
use quality::*;

use tokio::{runtime::Builder, task::JoinSet};

use crate::copilot::verify_copilot_yaml;

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = inputs::gather_inputs();

    if inputs.is_err() {
        github_actions::error!("Invalid or missing inputs.");
        exit(1);
    }

    let mut failed = false;

    let input_result = inputs.unwrap().clone();

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    let results = rt.block_on(async {
        let mut set = JoinSet::new();

        if input_result.check_dependabot {
            set.spawn(verify_dependabot_yaml(input_result.clone()));
            set.spawn(verify_dependabot_enabled(input_result.clone()));
        }
        set.spawn(verify_updates_yellr(input_result.clone()));
        set.spawn(verify_copilot_yaml(input_result.clone()));

        set.join_all().await
    });

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(input_result.step_summary_path)
        .unwrap();

    file.write_all("## Repository Compliance Report\n\n".as_bytes())
        .unwrap();

    for result in results {
        match result.into_markdown() {
            None => (),
            Some(s) => {
                file.write_all(s.as_bytes()).unwrap();
            }
        }
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
    Ok(())
}
