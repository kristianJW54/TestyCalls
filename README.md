### TestyCalls

Run the UI with `cargo run`. Press `r` or Enter to replay the sample demo,
`?` for commands, and `c` to inspect compiler output. The real Cargo runner and
test discovery are not connected yet.

For application logic, start with `Coordinator::handle_command` and
`Coordinator::handle_runner_event` in `src/coordinator.rs`.
See [the architecture guide](docs/architecture.md) for the file map and an
end-to-end example of where to connect real work.

## Fake tests

`tests/fake_tests.rs` contains 22 ordinary fake tests with nested names and varied
completion times. Some finish immediately; others sleep for up to five seconds.
They produce no custom progress output: your runner can consume Cargo's normal
test results through its subprocess pipes.

```sh
cargo test --test fake_tests
```

By default, 18 tests pass, 2 intentionally fail, and 2 are ignored. The failures
include an `assert_eq!` mismatch and a delayed `panic!`. Cargo exits with status
101 and prints failure details after the individual test results. Include ignored
tests as well with:

```sh
cargo test --test fake_tests -- --include-ignored
```

That runs all 22 tests: 20 pass and 2 intentionally fail (exit status 101).
The current UI uses a simulated runner. The future subprocess runner can read
these results and failure details from Cargo's output.

For a steady sequence of results, limit concurrency:

```sh
cargo test --test fake_tests -- --test-threads=2
```
