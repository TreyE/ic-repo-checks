use crate::{
    github_utils::{
        file_check, octocrab_repo_handler_for, octocrab_with_token_for, FileCheckResult,
        RateThrottle,
    },
    inputs::Inputs,
    results::CheckResult,
};

pub(crate) async fn verify_copilot_yaml(
    mut results: RateThrottle,
    inputs: Inputs,
) -> Vec<CheckResult> {
    let oc = octocrab_with_token_for(&inputs);
    let rh = octocrab_repo_handler_for(&oc, &inputs);
    let sem = results.acquire().await;
    let repo = rh.get().await.unwrap();
    if !repo.private.unwrap_or(false) {
        return vec![CheckResult::Ignore];
    }
    drop(sem);
    let _ = results.acquire().await;
    match file_check(&oc, &inputs, ".copilotignore").await {
        FileCheckResult::Found => vec![CheckResult::Pass(
            "Found a `.copilotignore` file for a private repository.".to_owned(),
        )],
        FileCheckResult::AccessDenied => vec![CheckResult::Failure(
            "Could not find a .copilotignore file for a private repository: Access Denied"
                .to_owned(),
        )],
        FileCheckResult::AccessForbidden => vec![CheckResult::Failure(
            "Could not find a .copilotignore file for a private repository: Access forbidden."
                .to_owned(),
        )],
        FileCheckResult::NotFound => vec![CheckResult::Failure(
            "Could not find a .copilotignore file for a private repository.".to_owned(),
        )],
        FileCheckResult::Error(_) => vec![CheckResult::Failure(
            "Could not find a .copilotignore file for a private repository.".to_owned(),
        )],
    }
}
