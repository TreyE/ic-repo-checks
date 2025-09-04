use octocrab::{models::hooks::Hook, OctocrabBuilder};

use crate::{inputs::Inputs, results::CheckResult};

static YELLR_WEBHOOK_1: &str =
    "https://us-central1-active-branches-report.cloudfunctions.net/webhook";
static YELLR_WEBHOOK_2: &str = "https://yellr.app/webhook";

fn hook_check(h: &Hook) -> bool {
    let url = h.config.url.as_str();
    let correct_content_type = h
        .config
        .content_type
        .eq(&Some(octocrab::models::hooks::ContentType::Json));
    let correct_url = url.eq(YELLR_WEBHOOK_1) || url.eq(YELLR_WEBHOOK_2);
    let does_on_create = h
        .events
        .iter()
        .any(|e| e.eq(&octocrab::models::webhook_events::WebhookEventType::Create));
    let does_on_delete = h
        .events
        .iter()
        .any(|e| e.eq(&octocrab::models::webhook_events::WebhookEventType::Delete));
    let does_on_pr_request_review = h
        .events
        .iter()
        .any(|e| e.eq(&octocrab::models::webhook_events::WebhookEventType::PullRequestReview));
    let does_on_pr = h
        .events
        .iter()
        .any(|e| e.eq(&octocrab::models::webhook_events::WebhookEventType::PullRequest));
    let does_on_pushes = h
        .events
        .iter()
        .any(|e| e.eq(&octocrab::models::webhook_events::WebhookEventType::Push));
    let does_on_workflow_runs = h
        .events
        .iter()
        .any(|e| e.eq(&octocrab::models::webhook_events::WebhookEventType::WorkflowRun));
    let does_correct_events = does_on_create
        && does_on_delete
        && does_on_pr_request_review
        && does_on_pr
        && does_on_pushes
        && does_on_workflow_runs;
    correct_content_type && correct_url && h.active && does_correct_events
}

pub(crate) async fn verify_updates_yellr(inputs: Inputs) -> Vec<CheckResult> {
    let ob = OctocrabBuilder::new().personal_token(inputs.access_token);
    let oc = ob.build().unwrap();
    let dependabot_check: octocrab::Result<octocrab::Page<Hook>> = oc
        .get(
            "/repos/".to_owned() + &inputs.repository + "/hooks",
            None::<&()>,
        )
        .await;

    match dependabot_check {
        Err(_) => vec![CheckResult::Failure(
            "Could not check if repository reports to Yellr: Request failure.".to_owned(),
        )],
        Ok(x) => match oc.all_pages(x).await {
            Err(_) => vec![CheckResult::Failure(
                "Could not check if repository reports to Yellr: Request failure.".to_owned(),
            )],
            Ok(y) => {
                if y.iter().any(hook_check) {
                    vec![CheckResult::Pass(
                        "Repository Reports to Yellr correctly".to_owned(),
                    )]
                } else {
                    vec![CheckResult::Failure(
                        "Repository does not report to Yellr.".to_owned(),
                    )]
                }
            }
        },
    }
}
