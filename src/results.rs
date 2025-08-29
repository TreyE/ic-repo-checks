pub(crate) enum CheckResult {
    Pass(String),
    Failure(String),
    Ignore,
}

impl CheckResult {
    pub(crate) fn into_markdown(&self) -> Option<String> {
        match self {
            CheckResult::Pass(p) => Some(format!("\u{2705} {}\n", p)),
            CheckResult::Failure(f) => Some(format!("\u{274c} {}\n", f)),
            CheckResult::Ignore => None,
        }
    }
}
