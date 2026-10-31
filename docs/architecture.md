# Where to write application logic

Start in `src/coordinator.rs`. It owns the app and runner tasks and connects them.
There are two methods intended for application decisions:

- `handle_command`: decide what happens when the user requests a refresh or run.
- `handle_runner_event`: process results, update your future test-tree model, and
  send the UI an update.

The coordinator's `event_loop` and `run` handle waiting and shutdown. You normally
do not need to edit them to add a feature.

## Message flow

```text
Keyboard → tui/input.rs → AppCommand → Coordinator::handle_command
                                              ↓
                                        RunnerCommand
                                              ↓
                                       ProcessRunner
                                              ↓
                                           AppEvent
                                              ↓
                              Coordinator::handle_runner_event
                                              ↓
                                     AppView::apply_event
                                              ↓
                                           redraw
```

`src/messages.rs` defines the shared vocabulary. These types have no Ratatui
dependencies. `AppEvent` is used on both sides of the coordinator bridge while
forwarding remains sufficient; introduce distinct runner/UI events if their
information later differs.

## Adding real work

1. Add `RefreshList` or `RunTests` to `RunnerCommand` in `messages.rs`.
2. Map the appropriate `AppCommand` to it in `Coordinator::handle_command`.
3. Handle that command in `ProcessRunner::run` in `runner.rs`. Put subprocess
   execution and output parsing in modules under `runner/` as they grow.
4. Send `AppEvent` values through `self.events`. For example, stdout can use
   `AppEvent::Output { kind: OutputKind::Results, line }`; compiler stderr uses
   `OutputKind::Compiler`, and parsed failure details use `OutputKind::Failures`.
5. Handle additional application decisions in `handle_runner_event` and display
   changes in `AppView::apply_event`.

Use asynchronous subprocess I/O and read stdout and stderr concurrently. Do not
print into the active terminal UI. On shutdown, the coordinator cancels the
runner; real child processes will need explicit cleanup or `kill_on_drop(true)`.

Currently both refresh and run deliberately map to `RunDemo`. All artificial
timing is isolated in `runner/demo.rs`. It emits Preparing → Compiling → Compiled,
then progress and completion. Pressing r again restarts it. No Cargo process is
executed and no test output is fabricated.

## UI map

| File | Responsibility |
| --- | --- |
| `src/main.rs` | Start the coordinator |
| `src/tui.rs` | Wait for terminal events, animation ticks, and coordinator updates |
| `src/tui/input.rs` | Map application keys to commands |
| `src/tui/terminal.rs` | Terminal setup and guaranteed restoration on drop |
| `src/tui/view.rs` | Own components, apply incoming events, compose the screen |
| `src/tui/view/input.rs` | Local UI navigation: focus, scrolling, popup keys, mouse |
| `src/tui/view/layout.rs` | Panel sizes and wide/narrow arrangement |
| `src/tui/view/header.rs` | Title, project path, list status |
| `src/tui/view/popups.rs` | Compiler output and help/tree popups |
| `src/tui/components/output.rs` | Scrollable output panel and scroll bounds |
| `src/tui/components/test_info.rs` | Run activity and progress bar |
| `src/tui/components/test_list.rs` | List discovery status and throbber |
| `src/tui/components/theme.rs` | Shared panel border styling |

UI components own display state such as scroll offsets and spinner frames.
Business decisions belong in the coordinator; execution belongs in the runner.
Animation ticks only advance throbbers, never simulate application progress.

## Checking changes

Run `cargo test --bin testycalls` for application tests. The separate
`tests/fake_tests.rs` suite intentionally contains failing tests; it is input
for the future runner, not a passing regression suite.

`target/` remains ignored by Git. Search and actual test-tree discovery remain
unimplemented; the tree popup states that no list is loaded.
