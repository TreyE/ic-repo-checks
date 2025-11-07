use std::{sync::Arc, time::Duration};

use http_body_util::BodyExt;
use octocrab::{repos::RepoHandler, Error, Octocrab, OctocrabBuilder};
use tokio::sync::{Semaphore, SemaphorePermit};

use crate::inputs::Inputs;

#[derive(Clone)]
pub(crate) struct RateThrottle {
    inner: Arc<Semaphore>,
}

#[allow(dead_code)]
pub(crate) enum FileCheckResult {
    Found,
    AccessDenied,
    AccessForbidden,
    NotFound,
    Error(Error),
}

#[allow(dead_code)]
pub(crate) enum GrabFileResult {
    File(bytes::Bytes),
    AccessDenied,
    AccessForbidden,
    NotFound,
    Error(Error),
}

static MAX_ACTIVE_REQUESTS: usize = 2;

impl RateThrottle {
    pub(crate) fn new() -> Self {
        let sem = Semaphore::new(MAX_ACTIVE_REQUESTS);
        RateThrottle {
            inner: Arc::new(sem),
        }
    }

    pub(crate) async fn acquire(&mut self) -> SemaphorePermit {
        let borrow = self.inner.acquire().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
        borrow.unwrap()
    }
}

pub(crate) fn octocrab_with_token_for(inputs: &Inputs) -> Octocrab {
    let ob = OctocrabBuilder::new().personal_token(inputs.token.clone());
    ob.build().unwrap()
}

pub(crate) fn octocrab_with_access_token_for(inputs: &Inputs) -> Octocrab {
    let ob = OctocrabBuilder::new().personal_token(inputs.access_token.clone());
    ob.build().unwrap()
}

pub(crate) fn octocrab_repo_handler_for<'a>(oc: &'a Octocrab, inputs: &Inputs) -> RepoHandler<'a> {
    let repo_name = inputs
        .repository
        .strip_prefix((inputs.repository_owner.clone() + "/").as_str())
        .unwrap();
    oc.repos(inputs.repository_owner.clone(), repo_name)
}

pub(crate) async fn file_check(oc: &Octocrab, inputs: &Inputs, file_path: &str) -> FileCheckResult {
    let rh = octocrab_repo_handler_for(oc, inputs);
    let dependabot_file = rh.raw_file(inputs.sha.clone(), file_path).await;
    match dependabot_file {
        Ok(x) => {
            // TODO: 401 is unauthorized
            if x.status().is_success() {
                FileCheckResult::Found
            } else if x.status().as_u16() == 401 {
                FileCheckResult::AccessDenied
            } else if x.status().as_u16() == 403 {
                FileCheckResult::AccessForbidden
            } else {
                FileCheckResult::NotFound
            }
        }
        Err(e) => FileCheckResult::Error(e),
    }
}

pub(crate) async fn grab_file(oc: &Octocrab, inputs: &Inputs, file_path: &str) -> GrabFileResult {
    let rh = octocrab_repo_handler_for(oc, inputs);
    let dependabot_file = rh.raw_file(inputs.sha.clone(), file_path).await;
    match dependabot_file {
        Ok(x) => {
            // TODO: 401 is unauthorized
            if x.status().is_success() {
                GrabFileResult::File(x.into_body().collect().await.unwrap().to_bytes())
            } else if x.status().as_u16() == 401 {
                GrabFileResult::AccessDenied
            } else if x.status().as_u16() == 403 {
                GrabFileResult::AccessForbidden
            } else {
                GrabFileResult::NotFound
            }
        }
        Err(e) => GrabFileResult::Error(e),
    }
}
