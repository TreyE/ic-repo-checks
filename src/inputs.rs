use std::collections::HashMap;

use github_actions::{
    BoolInputResult, InputResult, GITHUB_REPOSITORY, GITHUB_REPOSITORY_OWNER, GITHUB_SHA,
    GITHUB_STEP_SUMMARY,
};

enum InputValue {
    Str(String),
    Boolean(bool),
}

impl InputValue {
    unsafe fn as_str(&self) -> &str {
        match self {
            Self::Boolean(_) => panic!("not a boolean value!"),
            Self::Str(s) => s.as_str(),
        }
    }

    unsafe fn as_bool(&self) -> bool {
        match self {
            Self::Boolean(b) => b.clone(),
            Self::Str(_) => panic!("not a string value!"),
        }
    }
}

struct InputReader {
    inputs_read: HashMap<String, InputValue>,
    pub failures: Vec<String>,
}

impl InputReader {
    fn new() -> Self {
        InputReader {
            inputs_read: HashMap::new(),
            failures: Vec::new(),
        }
    }

    fn read_str_env(&mut self, key: &str) {
        let r_val = std::env::var(key);
        match r_val {
            Err(y) => match y {
                std::env::VarError::NotPresent => {
                    self.failures.push(format!("{} was not provided.", key))
                }
                std::env::VarError::NotUnicode(_z) => {
                    self.failures
                        .push(format!("{} was not properly encoded.", key));
                }
            },
            Ok(x) => {
                self.inputs_read
                    .insert(key.to_owned(), InputValue::Str(x.clone()));
            }
        };
    }

    fn read_str_input(&mut self, key: &str) {
        let r_val = github_actions::get_input(key);
        match r_val {
            Ok(x) => {
                self.inputs_read
                    .insert("INPUT_".to_owned() + key, InputValue::Str(x));
            }
            Err(y) => match y {
                InputResult::VarError(z) => match z {
                    std::env::VarError::NotPresent => self
                        .failures
                        .push(format!("{} was not provided as an input.", key)),
                    std::env::VarError::NotUnicode(_) => self
                        .failures
                        .push(format!("{} was not properly encoded as an input.", key)),
                },
            },
        }
    }

    fn read_bool_input(&mut self, key: &str) {
        let r_val = github_actions::get_bool_input(key);
        match r_val {
            Ok(x) => {
                self.inputs_read
                    .insert("INPUT_".to_owned() + key, InputValue::Boolean(x.clone()));
            }
            Err(y) => match y {
                BoolInputResult::TypeError => self.failures.push(format!(
                    "{} was provided as an input, but could not be converted to a boolean value.",
                    key
                )),
                BoolInputResult::VarError(z) => match z {
                    std::env::VarError::NotPresent => self
                        .failures
                        .push(format!("{} was not provided as an input.", key)),
                    std::env::VarError::NotUnicode(_) => self
                        .failures
                        .push(format!("{} was not properly encoded as an input.", key)),
                },
            },
        }
    }

    unsafe fn get_str_env(&self, key: &str) -> &str {
        self.inputs_read.get(key).unwrap().as_str()
    }

    unsafe fn get_str_input(&self, key: &str) -> &str {
        self.inputs_read
            .get(&("INPUT_".to_owned() + key))
            .unwrap()
            .as_str()
    }

    unsafe fn get_bool_input(&self, key: &str) -> bool {
        self.inputs_read
            .get(&("INPUT_".to_owned() + key))
            .unwrap()
            .as_bool()
    }
}

#[derive(Clone)]
pub(crate) struct Inputs {
    pub(crate) repository_owner: String,
    pub(crate) repository: String,
    pub(crate) token: String,
    pub(crate) sha: String,
    pub(crate) access_token: String,
    pub(crate) step_summary_path: String,
    pub(crate) check_dependabot: bool,
    pub(crate) check_yellr: bool,
    pub(crate) check_bundler_audit: bool,
}

static GITHUB_TOKEN: &str = "GITHUB_TOKEN";
static INPUT_ACCESS_TOKEN: &str = "ACCESS_TOKEN";
static INPUT_CHECK_DEPENDABOT: &str = "CHECK_DEPENDABOT";
static INPUT_CHECK_YELLR: &str = "CHECK_YELLR";
static INPUT_CHECK_BUNDLER_AUDIT: &str = "CHECK_BUNDLER_AUDIT";

pub(crate) fn gather_inputs() -> Result<Inputs, Vec<String>> {
    let mut input_reader = InputReader::new();

    input_reader.read_str_env(GITHUB_REPOSITORY_OWNER);
    input_reader.read_str_env(GITHUB_REPOSITORY);
    input_reader.read_str_env(GITHUB_SHA);
    input_reader.read_str_env(GITHUB_TOKEN);
    input_reader.read_str_input(INPUT_ACCESS_TOKEN);
    input_reader.read_str_env(GITHUB_STEP_SUMMARY);
    input_reader.read_bool_input(INPUT_CHECK_DEPENDABOT);
    input_reader.read_bool_input(INPUT_CHECK_YELLR);
    input_reader.read_bool_input(INPUT_CHECK_BUNDLER_AUDIT);

    if input_reader.failures.len() > 0 {
        return Err(input_reader.failures.clone());
    }

    unsafe {
        Ok(Inputs {
            repository_owner: input_reader.get_str_env(GITHUB_REPOSITORY_OWNER).to_owned(),
            repository: input_reader.get_str_env(GITHUB_REPOSITORY).to_owned(),
            token: input_reader.get_str_env(GITHUB_TOKEN).to_owned(),
            sha: input_reader.get_str_env(GITHUB_SHA).to_owned(),
            access_token: input_reader.get_str_input(INPUT_ACCESS_TOKEN).to_owned(),
            step_summary_path: input_reader.get_str_env(GITHUB_STEP_SUMMARY).to_owned(),
            check_dependabot: input_reader.get_bool_input(INPUT_CHECK_DEPENDABOT),
            check_yellr: input_reader.get_bool_input(INPUT_CHECK_YELLR),
            check_bundler_audit: input_reader.get_bool_input(INPUT_CHECK_BUNDLER_AUDIT),
        })
    }
}
