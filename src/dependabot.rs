use http::request::Builder;

use crate::{
    github_utils::{
        file_check, octocrab_with_access_token_for, octocrab_with_token_for, FileCheckResult,
        RateThrottle,
    },
    inputs::Inputs,
    results::CheckResult,
};

pub(crate) async fn verify_dependabot(requests: RateThrottle, inputs: Inputs) -> Vec<CheckResult> {
    vec![
        verify_dependabot_enabled(requests.clone(), inputs.clone()).await,
        verify_dependabot_yaml(requests, inputs.clone()).await,
    ]
}

async fn verify_dependabot_yaml(mut requests: RateThrottle, inputs: Inputs) -> CheckResult {
    let oc = octocrab_with_token_for(&inputs);
    let _ = requests.acquire().await;
    match file_check(&oc, &inputs, ".github/dependabot.yml").await {
        FileCheckResult::Found => CheckResult::Pass("Found a `.github/dependabot.yml`".to_owned()),
        FileCheckResult::AccessDenied => CheckResult::Failure(
            "Could not find a .github/dependabot.yml file: Access Denied".to_owned(),
        ),
        FileCheckResult::AccessForbidden => CheckResult::Failure(
            "Could not find a .github/dependabot.yml file: Access forbidden.".to_owned(),
        ),
        FileCheckResult::Error(_) => {
            CheckResult::Failure("Could not find a .github/dependabot.yml file.".to_owned())
        }
        FileCheckResult::NotFound => {
            CheckResult::Failure("Could not find a .github/dependabot.yml file.".to_owned())
        }
    }
}

async fn verify_dependabot_enabled(mut requests: RateThrottle, inputs: Inputs) -> CheckResult {
    let oc = octocrab_with_access_token_for(&inputs);
    let builder = Builder::new()
        .uri("/repos/".to_owned() + &inputs.repository + "/vulnerability-alerts")
        .method(http::Method::GET);
    let req = oc.build_request(builder, None::<&()>).unwrap();
    let _ = requests.acquire().await;
    let dependabot_check = oc.execute(req).await;
    match dependabot_check {
        Err(_) => CheckResult::Failure(
            "Could not check if dependabot was enabled: Request failure.".to_owned(),
        ),
        Ok(x) => {
            if x.status().is_success() {
                CheckResult::Pass("Dependabot is enabled".to_owned())
            } else if x.status().as_u16() == 401 {
                CheckResult::Failure(
                    "Could not check if dependabot was enabled: Access denied.".to_owned(),
                )
            } else if x.status().as_u16() == 403 {
                CheckResult::Failure(
                    "Could not check if dependabot was enabled: Access forbidden.".to_owned(),
                )
            } else {
                CheckResult::Failure(format!(
                    "Dependabot not enabled.  Endpoint returned {}.",
                    x.status().as_str()
                ))
            }
        }
    }
}
