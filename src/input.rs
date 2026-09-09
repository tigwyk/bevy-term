//! One keypress, as a Bevy message. The terminal writes them; the harness injects
//! them. Games never see a backend key type.

use bevy::prelude::Message;
use crossterm::event::KeyCode;

#[derive(Message, Clone, Copy, Debug, PartialEq, Eq)]
pub enum Key {
    Up,
    Down,
    Left,
    Right,
    Enter,
    Esc,
    Tab,
    Backspace,
    Char(char),
    F(u8),
    Other,
}

impl From<KeyCode> for Key {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Up => Key::Up,
            KeyCode::Down => Key::Down,
            KeyCode::Left => Key::Left,
            KeyCode::Right => Key::Right,
            KeyCode::Enter => Key::Enter,
            KeyCode::Esc => Key::Esc,
            KeyCode::Tab => Key::Tab,
            KeyCode::Backspace => Key::Backspace,
            KeyCode::Char(c) => Key::Char(c),
            KeyCode::F(n) => Key::F(n),
            _ => Key::Other,
        }
    }
}

impl Key {
    /// A script token: `esc`, `enter`, `space`, `tab`, `up`, `down`, `left`, `right`,
    /// `backspace`, `f1`..`f12`, or any single character.
    pub fn parse(token: &str) -> Option<Key> {
        Some(match token {
            "esc" => Key::Esc,
            "enter" => Key::Enter,
            "space" => Key::Char(' '),
            "tab" => Key::Tab,
            "up" => Key::Up,
            "down" => Key::Down,
            "left" => Key::Left,
            "right" => Key::Right,
            "backspace" => Key::Backspace,
            t if t.len() > 1 && t.starts_with('f') => Key::F(t[1..].parse().ok()?),
            t => {
                let mut chars = t.chars();
                let c = chars.next()?;
                if chars.next().is_some() {
                    return None;
                }
                Key::Char(c)
            }
        })
    }
}
