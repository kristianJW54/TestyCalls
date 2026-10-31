//! Ordinary fake tests for developing the TUI against Cargo's test output.

use std::{thread, time::Duration};

fn wait(milliseconds: u64) {
    thread::sleep(Duration::from_millis(milliseconds));
}

mod config {
    #[test]
    fn loads_defaults() {}

    #[test]
    fn reads_config_file() {
        super::wait(200);
    }

    #[test]
    fn rejects_invalid_config() {
        super::wait(100);
    }

    #[test]
    fn applies_environment_overrides() {
        super::wait(300);
    }
}

mod parser {
    #[test]
    fn accepts_empty_input() {}

    #[test]
    fn parses_single_result() {
        super::wait(150);
    }

    #[test]
    fn parses_multiple_results() {
        super::wait(400);
    }

    #[test]
    fn handles_nested_test_names() {
        super::wait(250);
    }

    #[test]
    fn handles_large_input() {
        super::wait(2_000);
    }
}

mod runner {
    #[test]
    fn starts_process() {
        super::wait(500);
    }

    #[test]
    fn collects_results() {
        super::wait(1_000);
    }

    #[test]
    fn waits_for_slow_process() {
        super::wait(3_000);
    }

    #[test]
    fn handles_process_completion() {
        super::wait(4_000);
    }

    #[test]
    fn runs_long_job() {
        super::wait(5_000);
    }
}

mod ui {
    #[test]
    fn selects_first_test() {}

    #[test]
    fn moves_selection_down() {
        super::wait(100);
    }

    #[test]
    fn filters_test_names() {
        super::wait(350);
    }

    #[test]
    fn updates_test_status() {
        super::wait(750);
    }

    #[test]
    #[ignore = "example ignored test"]
    fn renders_large_suite() {
        super::wait(1_000);
    }

    #[test]
    #[ignore = "example ignored test"]
    fn restores_previous_session() {}
}

mod failures {
    #[test]
    fn unexpected_result_count() {
        let actual = 3;
        assert_eq!(actual, 5, "intentional fixture failure");
    }

    #[test]
    fn slow_request_times_out() {
        super::wait(3_000);
        panic!("intentional fixture failure: request timed out");
    }
}
