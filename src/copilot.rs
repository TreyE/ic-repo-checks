use octocrab::OctocrabBuilder;

use crate::{inputs::Inputs, results::CheckResult};

pub(crate) async fn verify_copilot_yaml(inputs: Inputs) -> Vec<CheckResult> {
    let ob = OctocrabBuilder::new().personal_token(inputs.token.as_ref());
    let repo_name = inputs
        .repository
        .strip_prefix((inputs.repository_owner.clone() + "/").as_str())
        .unwrap();
    let oc = ob.build().unwrap();
    let rh = oc.repos(inputs.repository_owner.clone(), repo_name);
    let repo = rh.get().await.unwrap();
    if !repo.private.unwrap_or(false) {
        return vec![CheckResult::Ignore];
    }
    let copilot_ignore_file = rh.raw_file(inputs.sha.clone(), ".copilotignore").await;
    match copilot_ignore_file {
        Ok(x) => {
            if x.status().is_success() {
                vec![CheckResult::Pass(
                    "Found a `.copilotignore` file for a private repository.".to_owned(),
                )]
            } else if x.status().as_u16() == 401 {
                vec![CheckResult::Failure(
                    "Could not find a .copilotignore file for a private repository: Access Denied"
                        .to_owned(),
                )]
            } else if x.status().as_u16() == 403 {
                vec![CheckResult::Failure(
                    "Could not find a .copilotignore file for a private repository: Access forbidden.".to_owned(),
                )]
            } else {
                vec![CheckResult::Failure(
                    "Could not find a .copilotignore file for a private repository.".to_owned(),
                )]
            }
        }
        Err(_) => vec![CheckResult::Failure(
            "Could not find a .copilotignore file for a private repository.".to_owned(),
        )],
    }
}
