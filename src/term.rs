//! The terminal seam. `Term` wraps a ratatui `Terminal` over either a real tty
//! (crossterm) or an in-memory `TestBackend`. Game draw systems take
//! `ResMut<Term>` and call `draw`; they never know which.

use std::io::{self, Stdout};
use std::time::Duration;

use bevy::prelude::*;
use crossterm::cursor::{Hide, Show};
use crossterm::event::{self, Event, KeyEventKind};
use crossterm::execute;
use crossterm::terminal::{
    EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode,
};
use ratatui::backend::{CrosstermBackend, TestBackend};
use ratatui::{Frame, Terminal};

use crate::input::Key;

enum Inner {
    Tty(Terminal<CrosstermBackend<Stdout>>),
    Test(Terminal<TestBackend>),
}

#[derive(Resource)]
pub struct Term(Inner);

impl Term {
    /// Render one frame. Draw errors are non-fatal: the next frame repaints.
    pub fn draw(&mut self, f: impl FnOnce(&mut Frame<'_>)) {
        match &mut self.0 {
            Inner::Tty(t) => {
                let _ = t.draw(f);
            }
            Inner::Test(t) => {
                let _ = t.draw(f);
            }
        }
    }

    /// The screen as text, one line per row, trailing spaces trimmed. Headless only;
    /// a real tty has no readable buffer, so this is `None` there.
    pub fn text(&self) -> Option<String> {
        let Inner::Test(t) = &self.0 else { return None };
        let buf = t.backend().buffer();
        let area = buf.area;
        let mut out = String::new();
        for y in 0..area.height {
            let line: String = (0..area.width).map(|x| buf[(x, y)].symbol()).collect();
            out.push_str(line.trim_end());
            out.push('\n');
        }
        Some(out)
    }
}

impl Drop for Term {
    fn drop(&mut self) {
        if let Inner::Tty(_) = self.0 {
            let _ = execute!(io::stdout(), Show, LeaveAlternateScreen);
            let _ = disable_raw_mode();
        }
    }
}

/// A real terminal: raw mode, alternate screen, keys polled into `Key` messages
/// each `PreUpdate`. Add it to [`crate::app`].
pub struct TerminalPlugin;

impl Plugin for TerminalPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Key>()
            .add_systems(Startup, setup_tty)
            .add_systems(PreUpdate, read_keys);
    }
}

fn setup_tty(mut commands: Commands) {
    enable_raw_mode().expect("failed to enter raw mode");
    execute!(io::stdout(), EnterAlternateScreen, Hide).expect("failed to enter alternate screen");
    let terminal =
        Terminal::new(CrosstermBackend::new(io::stdout())).expect("ratatui terminal");
    commands.insert_resource(Term(Inner::Tty(terminal)));
}

// ratatui re-reads the size from `frame.area()` each frame, so resizes need no handling.
fn read_keys(mut keys: MessageWriter<Key>) {
    while event::poll(Duration::ZERO).unwrap_or(false) {
        match event::read() {
            Ok(Event::Key(k)) if k.kind == KeyEventKind::Press => {
                keys.write(k.code.into());
            }
            Ok(_) => {}
            Err(_) => break,
        }
    }
}

/// No tty: an in-memory screen of the given size. Keys come from `harness::press`.
pub struct HeadlessPlugin {
    pub width: u16,
    pub height: u16,
}

impl Default for HeadlessPlugin {
    fn default() -> Self {
        Self { width: 80, height: 24 }
    }
}

impl Plugin for HeadlessPlugin {
    fn build(&self, app: &mut App) {
        let terminal =
            Terminal::new(TestBackend::new(self.width, self.height)).expect("test terminal");
        app.add_message::<Key>()
            .insert_resource(Term(Inner::Test(terminal)));
    }
}
