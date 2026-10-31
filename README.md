# TestyCalls

**A terminal workspace for Rust tests.**

TestyCalls is being built to help you find tests, run them, and inspect results
without losing your place in the terminal. Built with Rust, Ratatui, and Tokio.

> **Early development:** the interface and task communication are working.
> Running and refreshing currently play a simulated demo. Cargo test discovery,
> real execution, and catalogue search are still being built.

## A place for every part of a test run

- **Run progress:** animated preparation, compilation, and completion states.
- **Results and failures:** independently scrollable panels, side by side on
  wide terminals and stacked on narrow ones.
- **Compiler output:** a large, scrollable popup available when you need it.
- **Project context:** the working directory and test-list status beneath the title.
- **Keyboard and mouse navigation:** switch panels, scroll through output, and
  open command help without leaving the app.

The demo animates the status and progress widgets. Output panels remain empty
until real runner output is connected; the test-tree popup is a placeholder.

## Get started

Use a recent stable Rust toolchain and a terminal with Unicode support.

```sh
git clone https://github.com/kristianJW54/TestyCalls.git
cd TestyCalls
cargo run
```

Press **r** or **Enter** to play the sample run:

```text
Preparing → Compiling → Catalogue compiled → Running tests → Completed
```

Press **q** to quit. The terminal is restored when the app exits.

## Controls

| Key | Action |
| --- | --- |
| `?` | Open command help |
| `r` | Refresh request; currently restarts the demo |
| `Enter` | Run request; currently restarts the demo |
| `Tab` / `Shift+Tab` | Switch between results and failures |
| `j` / `k` or `↓` / `↑` | Scroll the focused output panel |
| `Page Down` / `Page Up` | Scroll a page |
| `Home` / `End` | Jump to the start or end |
| Mouse wheel | Scroll the output panel under the pointer |
| `c` | Open compiler messages; press again to close |
| `t` | Open the test-tree placeholder |
| `Esc` | Close the active popup |
| `q` | Quit |

While a popup is open, input stays with it. The compiler popup supports the
same scrolling keys as the output panels.

## What comes next

- Discover packages and test targets through Cargo metadata.
- Build a catalogue with package, target, kind, source path, and full test name.
- Refresh the catalogue manually and retain the previous list if discovery fails.
- Browse a test tree and fuzzy-find tests by their full catalogue path.
- Execute selected tests and route results, failures, and compiler messages to
  their respective views.

## Working on the app

Application decisions, execution, and rendering live in separate modules:

```text
App → command → Coordinator → request → Runner
App ← update  ← Coordinator ← event   ← Runner
```

| Start here | Responsibility |
| --- | --- |
| [`src/coordinator.rs`](src/coordinator.rs) | Handle application commands and runner events |
| [`src/messages.rs`](src/messages.rs) | Define the commands and events exchanged between tasks |
| [`src/runner.rs`](src/runner.rs) | Execute requested work |
| [`src/runner/demo.rs`](src/runner/demo.rs) | Isolate the current simulated run |
| [`src/test_tree.rs`](src/test_tree.rs) | Starting types for target identities and the test catalogue |
| [`src/tui.rs`](src/tui.rs) | Wait for user input, updates, and animation ticks |
| [`src/tui/view.rs`](src/tui/view.rs) | Apply updates and compose the screen |
| [`src/tui/components/`](src/tui/components/) | Render individual widgets |

See the [architecture guide](docs/architecture.md) for the full file map and
where to connect real subprocess work.

## Development checks

```sh
cargo check --all-targets
cargo test --bin testycalls
```

Application tests cover message routing, shutdown, demo transitions, scrolling,
and rendering at different terminal sizes.

### Sample tests for the runner

[`tests/fake_tests.rs`](tests/fake_tests.rs) contains 22 sample tests with varied
durations and nested names. **Two intentionally fail and two are ignored**;
running the entire test suite is therefore expected to return a failure.

```sh
# List the sample tests without executing them.
cargo test --test fake_tests -- --list

# Run them with a steady stream of completions.
cargo test --test fake_tests -- --test-threads=2

# Include the ignored tests as well.
cargo test --test fake_tests -- --include-ignored
```

On Windows, Cargo may be unable to rebuild this project's executable while the
TUI is running. Quit the app first, or use a separate build directory:

```sh
cargo test --target-dir target/test-runner -- --list
```

All `target/` directories are ignored by Git, including that separate build directory.

## License

[Apache License 2.0](LICENSE).
