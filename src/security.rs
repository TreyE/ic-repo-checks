use http::request::Builder;
use octocrab::OctocrabBuilder;

use crate::{check_result::CheckResult, inputs::Inputs};

pub(crate) async fn verify_dependabot_yaml(inputs: Inputs) -> CheckResult {
    let ob = OctocrabBuilder::new().personal_token(inputs.token.as_ref());
    let repo_name = inputs
        .repository
        .strip_prefix((inputs.repository_owner.clone() + "/").as_str())
        .unwrap();
    let oc = ob.build().unwrap();
    let rh = oc.repos(inputs.repository_owner.clone(), repo_name);
    let dependabot_file = rh
        .raw_file(inputs.sha.clone(), ".github/dependabot.yml")
        .await;
    match dependabot_file {
        Ok(x) => {
            // TODO: 401 is unauthorized
            if x.status().is_success() {
                CheckResult::Pass
            } else if x.status().as_u16() == 401 {
                CheckResult::Failure(
                    "Could not find a .github/dependabot.yml file: Access Denied.".to_owned(),
                )
            } else if x.status().as_u16() == 403 {
                CheckResult::Failure(
                    "Could not find a .github/dependabot.yml file: Access forbidden.".to_owned(),
                )
            } else {
                CheckResult::Failure("Could not find a .github/dependabot.yml file.".to_owned())
            }
        }
        Err(_) => CheckResult::Failure("Could not find a .github/dependabot.yml file.".to_owned()),
    }
}

pub(crate) async fn verify_dependabot_enabled(inputs: Inputs) -> CheckResult {
    let ob = OctocrabBuilder::new().personal_token(inputs.token.as_ref());
    let oc = ob.build().unwrap();
    let builder = Builder::new()
        .uri("/repos/".to_owned() + &inputs.repository + "/vulnerability-alerts")
        .method(http::Method::GET);
    let req = oc.build_request(builder, None::<&()>).unwrap();
    let dependabot_check = oc.execute(req).await;
    match dependabot_check {
        Err(_) => CheckResult::Failure(
            "Could not check if dependabot was enabled: Request failure.".to_owned(),
        ),
        Ok(x) => {
            if x.status().is_success() {
                CheckResult::Pass
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
