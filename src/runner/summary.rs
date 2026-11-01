#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TestSummary {
    pub passed: usize,
    pub failed: usize,
    pub ignored: usize,
    pub measured: usize,
    pub filtered: usize,
    pub seconds: f64,
}

impl TestSummary {
    pub fn parse(line: &str) -> Option<Self> {
        let text = line.trim().strip_prefix("test result: ")?;
        let (_, fields) = text.split_once(". ")?;
        let mut fields = fields.split("; ");
        let passed = fields.next()?.strip_suffix(" passed")?.parse().ok()?;
        let failed = fields.next()?.strip_suffix(" failed")?.parse().ok()?;
        let ignored = fields.next()?.strip_suffix(" ignored")?.parse().ok()?;
        let measured = fields.next()?.strip_suffix(" measured")?.parse().ok()?;
        let filtered = fields.next()?.strip_suffix(" filtered out")?.parse().ok()?;
        let seconds: f64 = fields
            .next()?
            .strip_prefix("finished in ")?
            .strip_suffix('s')?
            .parse()
            .ok()?;
        if !seconds.is_finite() || seconds < 0.0 {
            return None;
        }
        Some(Self {
            passed,
            failed,
            ignored,
            measured,
            filtered,
            seconds,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_success_failure_and_rejects_incomplete_lines() {
        let summary = TestSummary::parse("test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 0.00s").unwrap();
        assert_eq!(
            (summary.passed, summary.failed, summary.filtered),
            (1, 0, 8)
        );
        assert_eq!(summary.seconds, 0.0);
        assert!(TestSummary::parse("test result: FAILED. 8 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 4.00s").is_some());
        assert!(TestSummary::parse("test result: ok.").is_none());
    }
}
