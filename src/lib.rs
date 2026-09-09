//! Terminal games on headless Bevy + ratatui.
//!
//! Shape of every frame: `Key` messages → game input systems mutate resources →
//! a draw system paints ratatui widgets through [`Term`]. The same app runs on a
//! real tty ([`TerminalPlugin`]) or in a test ([`HeadlessPlugin`] via [`harness`]),
//! where the screen reads back as text.

pub mod harness;
pub mod input;
pub mod rng;
pub mod save;
pub mod screen;
pub mod term;

use std::time::Duration;

use bevy::app::{App, ScheduleRunnerPlugin};
use bevy::prelude::{MinimalPlugins, PluginGroup};
use bevy::state::app::StatesPlugin;

/// A headless Bevy app ticking at 30 fps with states enabled. Add [`TerminalPlugin`]
/// and the game's plugins on top. Tests use [`harness::headless`] instead.
pub fn app() -> App {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f32(1.0 / 30.0))),
        StatesPlugin,
    ));
    app
}

pub use input::Key;
pub use rng::Rng;
pub use save::SaveDir;
pub use screen::ScreenStack;
pub use term::{HeadlessPlugin, Term, TerminalPlugin};
