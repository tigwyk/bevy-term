# bevy-term

`bevy-term` is a small toolkit for building terminal games with a headless Bevy
app and [ratatui](https://ratatui.rs/). Your game is made of ordinary Bevy
resources and systems: input arrives as `Key` messages, and a draw system renders
each frame through `Term`. The same game systems can run in a real terminal or
against an in-memory screen in tests.

## Add the dependency

For a local checkout, add the package to your application's `Cargo.toml` (adjust
the path to the checkout):

```toml
[dependencies]
bevy-term = { path = "../bevy-term" }
```

If your application uses `bevy` or `ratatui` types directly, add those crates as
direct dependencies too. `bevy-term` currently uses Bevy 0.19 and ratatui 0.30.

## Build a terminal app

Add `TerminalPlugin` and your game plugin to the app returned by `bevy_term::app()`.
The helper enables Bevy states and runs the app at 30 frames per second.
`TerminalPlugin` enters raw mode and the alternate screen, then writes pressed
keys as `Key` messages. Your systems can read those messages and draw using
ratatui widgets:

```rust
use bevy::prelude::*;
use bevy_term::{Key, ScreenStack, Term, TerminalPlugin};
use ratatui::widgets::Paragraph;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Main,
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScreenStack::new(Screen::Main))
            .add_systems(PreUpdate, handle_input)
            .add_systems(Update, draw);
    }
}

fn handle_input(
    mut keys: MessageReader<Key>,
    mut exit: MessageWriter<AppExit>,
) {
    for key in keys.read() {
        if *key == Key::Esc {
            exit.write_default();
        }
    }
}

fn draw(mut term: ResMut<Term>) {
    term.draw(|frame| {
        frame.render_widget(Paragraph::new("Hello, terminal! Press Esc to quit."), frame.area());
    });
}

fn main() {
    bevy_term::app()
        .add_plugins((TerminalPlugin, GamePlugin))
        .run();
}
```

`Term::draw` accepts a closure over a ratatui frame. The game does not need to
know whether that frame is backed by a real terminal or the test backend.

## Test without a terminal

Use `harness::headless` to create an 80×24 in-memory terminal, install your game,
and draw its initial frame. Inject input with `harness::press` or a space-separated
script with `harness::run_script`; inspect rendered text with `harness::screen`
or `harness::shows`.

```rust
use bevy::prelude::*;
use bevy_term::{ScreenStack, Term};
use ratatui::widgets::Paragraph;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Screen {
    Main,
}

fn draw(mut term: ResMut<Term>) {
    term.draw(|frame| {
        frame.render_widget(Paragraph::new("Hello, terminal!"), frame.area());
    });
}

#[test]
fn main_screen_is_rendered() {
    let app = bevy_term::harness::headless(|app| {
        app.insert_resource(ScreenStack::new(Screen::Main))
            .add_systems(Update, draw);
    });

    assert!(bevy_term::harness::shows(&app, "Hello, terminal!"));
}
```

The harness also provides `wants_exit` to check whether the app requested exit.
When constructing your own headless app, configure `HeadlessPlugin { width, height }`
alongside `MinimalPlugins` and `StatesPlugin` to choose a different virtual screen size.

## Included utilities

- **`Key`** — backend-independent messages for arrows, Enter, Escape, Tab,
  Backspace, characters, function keys, and other keys.
- **`ScreenStack<S>`** — a small navigation stack. `pop` leaves the root screen
  in place; `push`, `reset`, `top`, and `contains` are also available.
- **`Rng`** — a seeded ChaCha8 random-number resource with helpers for ranges,
  chances, weighted choices, and picking from a slice.
- **`SaveDir`** — load and save Serde values as RON files in a per-game data
  directory. Use `SaveDir::for_game("your-game")` and insert it as a resource.

Run the package tests with:

```sh
cargo test
```
