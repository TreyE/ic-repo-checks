pub(crate) enum CheckResult {
    Pass(String),
    Failure(String),
}

impl CheckResult {
    pub(crate) fn into_markdown(&self) -> String {
        match self {
            CheckResult::Pass(p) => format!("\u{2705} {}\n", p),
            CheckResult::Failure(f) => format!("\u{274c} {}\n", f),
        }
    }
}
