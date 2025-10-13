use octocrab::Octocrab;

use crate::{
    github_utils::{
        file_check, grab_file, octocrab_with_token_for, FileCheckResult, GrabFileResult,
    },
    inputs::Inputs,
    results::CheckResult,
};

pub(crate) async fn verify_rails_projects(inputs: Inputs) -> Vec<CheckResult> {
    let oc = octocrab_with_token_for(&inputs);
    let mut results = Vec::new();
    if inputs.check_bundler_audit {
        results.push(verify_bundler_audit(&oc, &inputs).await);
    }
    results
}

async fn verify_bundler_audit(oc: &Octocrab, inputs: &Inputs) -> CheckResult {
    let gl_file = grab_file(&oc, &inputs, "Gemfile.lock").await;
    let gem_file = grab_file(&oc, &inputs, "Gemfile").await;
    match (gem_file, gl_file) {
        (GrabFileResult::NotFound, GrabFileResult::NotFound) => CheckResult::Ignore,
        (GrabFileResult::File(_), _) => check_for_bundler_audit_yaml(oc, inputs).await,
        (_, GrabFileResult::File(_)) => check_for_bundler_audit_yaml(oc, inputs).await,
        (GrabFileResult::AccessDenied, _) => CheckResult::Failure(
            "Could not check for a Gemfile.lock file: Access denied.".to_owned(),
        ),
        (GrabFileResult::AccessForbidden, _) => CheckResult::Failure(
            "Could not check for a Gemfile.lock file: Access forbidden.".to_owned(),
        ),
        _ => CheckResult::Failure(
            "Could not check if we need bundler audit: Request failure.".to_owned(),
        ),
    }
}

async fn check_for_bundler_audit_yaml(oc: &Octocrab, inputs: &Inputs) -> CheckResult {
    match file_check(&oc, &inputs, ".bundler-audit.yml").await {
        FileCheckResult::Found => {
            CheckResult::Pass("Found a `.bundler-audit.yml` file.".to_owned())
        }
        FileCheckResult::AccessDenied => CheckResult::Failure(
            "Could not find a `.bundler-audit.yml` file: Access Denied".to_owned(),
        ),
        FileCheckResult::AccessForbidden => CheckResult::Failure(
            "Could not find a `.bundler-audit.yml` file: Access forbidden.".to_owned(),
        ),
        FileCheckResult::NotFound => {
            CheckResult::Failure("Could not find a `.bundler-audit.yml` file.".to_owned())
        }
        FileCheckResult::Error(_) => CheckResult::Failure(
            "Could not find a `.bundler-audit.yml` file: request failed.".to_owned(),
        ),
    }
}
