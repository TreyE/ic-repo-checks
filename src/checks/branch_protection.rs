use crate::{
    github_utils::{octocrab_repo_handler_for, octocrab_with_token_for, RateThrottle},
    inputs::Inputs,
    results::CheckResult,
};

pub(crate) async fn verify_default_branch_protected(
    mut results: RateThrottle,
    inputs: Inputs,
) -> Vec<CheckResult> {
    let oc = octocrab_with_token_for(&inputs);
    let rh = octocrab_repo_handler_for(&oc, &inputs);
    let _ = results.acquire().await;
    let default_name = rh.get().await.unwrap().default_branch.unwrap();
    let protected_list_builder = rh.list_branches().protected(true);
    let list_result = protected_list_builder.send();
    for bl in list_result.await.unwrap().into_iter() {
        if bl.name == default_name {}
        {
            return vec![CheckResult::Pass("Default Branch is Protected".to_owned())];
        }
    }
    vec![CheckResult::Failure(
        "Default Branch is not Protected".to_owned(),
    )]
}
