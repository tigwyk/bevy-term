//! Drive the whole app without a terminal: inject keys, read the screen back as
//! text. Backs `cargo test` and a `--script "8 5 esc"` CLI flag.

use bevy::prelude::*;
use bevy::state::app::StatesPlugin;

use crate::input::Key;
use crate::term::{HeadlessPlugin, Term};

/// `MinimalPlugins` + states + an 80x24 headless screen, then the game's own
/// `build`, then one idle frame so the first screen is drawn.
pub fn headless(build: impl FnOnce(&mut App)) -> App {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, StatesPlugin, HeadlessPlugin::default()));
    build(&mut app);
    app.update();
    app
}

/// One keypress, then one frame.
pub fn press(app: &mut App, key: Key) {
    app.world_mut().write_message(key);
    app.update();
}

/// The screen as text. See [`Term::text`].
pub fn screen(app: &App) -> String {
    app.world().resource::<Term>().text().unwrap_or_default()
}

pub fn shows(app: &App, text: &str) -> bool {
    screen(app).contains(text)
}

/// Whether an exit was requested (consumes any pending `AppExit`).
pub fn wants_exit(app: &mut App) -> bool {
    app.should_exit().is_some()
}

/// Presses every token of a space-separated key script (see [`Key::parse`]).
/// Unknown tokens are reported on stderr and skipped.
pub fn run_script(app: &mut App, script: &str) {
    for token in script.split_whitespace() {
        match Key::parse(token) {
            Some(key) => press(app, key),
            None => eprintln!("unknown key token '{token}'"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::screen::ScreenStack;
    use ratatui::widgets::Paragraph;

    #[derive(Clone, Copy, PartialEq, Debug)]
    enum Screen {
        Main,
        Sub,
    }

    fn input(
        mut keys: MessageReader<Key>,
        mut stack: ResMut<ScreenStack<Screen>>,
        mut exit: MessageWriter<AppExit>,
    ) {
        for k in keys.read() {
            match k {
                Key::Esc => {
                    stack.pop();
                }
                Key::Char('1') => stack.push(Screen::Sub),
                Key::Char('q') => {
                    exit.write_default();
                }
                _ => {}
            }
        }
    }

    fn draw(mut term: ResMut<Term>, stack: Res<ScreenStack<Screen>>) {
        let label = format!("{:?}", stack.top());
        term.draw(|f| f.render_widget(Paragraph::new(label), f.area()));
    }

    fn game(app: &mut App) {
        app.insert_resource(ScreenStack::new(Screen::Main))
            .add_systems(PreUpdate, input)
            .add_systems(Update, draw);
    }

    #[test]
    fn keys_move_screens_and_the_screen_reads_back() {
        let mut app = headless(game);
        assert!(shows(&app, "Main"));
        run_script(&mut app, "1");
        assert!(shows(&app, "Sub"));
        press(&mut app, Key::Esc);
        assert!(shows(&app, "Main"));
        // Esc at the root stays at the root.
        press(&mut app, Key::Esc);
        assert_eq!(app.world().resource::<ScreenStack<Screen>>().top(), Screen::Main);
        assert!(!wants_exit(&mut app));
        press(&mut app, Key::Char('q'));
        assert!(wants_exit(&mut app));
    }

    #[test]
    fn script_tokens_parse() {
        assert_eq!(Key::parse("esc"), Some(Key::Esc));
        assert_eq!(Key::parse("f2"), Some(Key::F(2)));
        assert_eq!(Key::parse("space"), Some(Key::Char(' ')));
        assert_eq!(Key::parse("x"), Some(Key::Char('x')));
        assert_eq!(Key::parse("xy"), None);
    }
}
