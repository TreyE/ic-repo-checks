use github_actions::{
    issue_command, BoolInputResult, GITHUB_REPOSITORY, GITHUB_REPOSITORY_OWNER, GITHUB_SHA,
    GITHUB_STEP_SUMMARY,
};

#[derive(Clone)]
pub(crate) struct Inputs {
    pub(crate) repository_owner: String,
    pub(crate) repository: String,
    pub(crate) token: String,
    pub(crate) sha: String,
    pub(crate) access_token: String,
    pub(crate) step_summary_path: String,
    pub(crate) check_dependabot: bool,
}

static GITHUB_TOKEN: &str = "GITHUB_TOKEN";
static INPUT_ACCESS_TOKEN: &str = "INPUT_ACCESS_TOKEN";
static INPUT_CHECK_DEPENDABOT: &str = "CHECK_DEPENDABOT";

fn read_input(input_name: &str) -> Result<String, String> {
    let r_val = std::env::var(input_name);
    match r_val {
        Err(y) => match y {
            std::env::VarError::NotPresent => Err(format!("{} was not provided.", input_name)),
            std::env::VarError::NotUnicode(_z) => {
                Err(format!("{} was not properly encoded.", input_name))
            }
        },
        Ok(x) => Ok(x),
    }
}

fn read_boolean_input(input_name: &str) -> Result<bool, String> {
    let r_val = github_actions::get_bool_input(input_name);
    match r_val {
        Ok(x) => Ok(x),
        Err(y) => match y {
            BoolInputResult::TypeError => Err(format!(
                "{} was provided, but could not be converted to a boolean value.",
                input_name
            )),
            BoolInputResult::VarError(z) => match z {
                std::env::VarError::NotPresent => Err(format!("{} was not provided.", input_name)),
                std::env::VarError::NotUnicode(_) => {
                    Err(format!("{} was not properly encoded.", input_name))
                }
            },
        },
    }
}

pub(crate) fn gather_inputs() -> Result<Inputs, Vec<String>> {
    let r_owner = read_input(GITHUB_REPOSITORY_OWNER);

    let mut failed = false;

    let mut failures = Vec::new();

    if r_owner.is_err() {
        failures.push(r_owner.clone().expect_err("should be an error"));
        failed = true;
    }
    let r_repo = read_input(GITHUB_REPOSITORY);
    if r_repo.is_err() {
        failures.push(r_repo.clone().expect_err("should be an error"));
        failed = true;
    }
    let r_sha = read_input(GITHUB_SHA);
    if r_sha.is_err() {
        failures.push(r_sha.clone().expect_err("should be an error"));
        failed = true;
    }
    let r_token = read_input(GITHUB_TOKEN);
    if r_token.is_err() {
        failures.push(r_token.clone().expect_err("should be an error"));
        failed = true;
    }

    let a_token = read_input(INPUT_ACCESS_TOKEN);
    if a_token.is_err() {
        failures.push(a_token.clone().expect_err("should be an error"));
        failed = true;
    }

    let ss_path = read_input(GITHUB_STEP_SUMMARY);
    if ss_path.is_err() {
        failures.push(ss_path.clone().expect_err("should be an error"));
        failed = true;
    }

    let check_dependabot_val_result = read_boolean_input(INPUT_CHECK_DEPENDABOT);
    if check_dependabot_val_result.is_err() {
        failures.push(
            check_dependabot_val_result
                .clone()
                .expect_err("should be an error"),
        );
        failed = true;
    }

    if failed {
        return Err(failures);
    }

    Ok(Inputs {
        repository_owner: r_owner.unwrap(),
        repository: r_repo.unwrap(),
        token: r_token.unwrap(),
        sha: r_sha.unwrap(),
        access_token: a_token.unwrap(),
        step_summary_path: ss_path.unwrap(),
        check_dependabot: check_dependabot_val_result.unwrap(),
    })
}
