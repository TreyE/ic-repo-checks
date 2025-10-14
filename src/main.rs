use std::{error::Error, fs::OpenOptions, io::Write, process::exit, sync::Arc};

mod github_utils;

mod copilot;

mod inputs;

mod results;

mod quality;

mod rails_projects;

mod dependabot;
use dependabot::*;
use github_actions::issue_command;
use quality::*;

use tokio::{runtime::Builder, sync::Semaphore, task::JoinSet};

use crate::{
    copilot::verify_copilot_yaml, github_utils::RateThrottle,
    rails_projects::verify_rails_projects, results::CheckResult,
};

fn main() -> Result<(), Box<dyn Error>> {
    let inputs = inputs::gather_inputs();

    if inputs.is_err() {
        github_actions::error!("Invalid or missing inputs.");
        exit(1);
    }

    let mut failed = false;

    let input_result = inputs.unwrap().clone();

    let rt = Builder::new_multi_thread().enable_all().build().unwrap();

    let requests = RateThrottle::new();

    let results = rt.block_on(async {
        let mut set = JoinSet::new();

        if input_result.check_dependabot {
            set.spawn(verify_dependabot(requests.clone(), input_result.clone()));
        }
        if input_result.check_yellr {
            set.spawn(verify_updates_yellr(requests.clone(), input_result.clone()));
        }
        set.spawn(verify_copilot_yaml(requests.clone(), input_result.clone()));
        /*set.spawn(verify_rails_projects(
            requests.clone(),
            input_result.clone(),
        ));*/

        set.join_all()
            .await
            .iter()
            .flat_map(|i| i)
            .map(|i| (*i).to_owned())
            .collect::<Vec<CheckResult>>()
    });

    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(input_result.step_summary_path)
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
