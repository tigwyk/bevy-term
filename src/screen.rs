//! Screen navigation as a stack. Esc pops, never below the root, so the player can
//! never be stranded. Modal screens (an unanswered event, combat) are the game's
//! call: it simply doesn't pop them.

use bevy::prelude::Resource;

#[derive(Resource, Debug)]
pub struct ScreenStack<S>(Vec<S>);

impl<S: Copy + Send + Sync + 'static> ScreenStack<S> {
    pub fn new(root: S) -> Self {
        Self(vec![root])
    }

    pub fn top(&self) -> S {
        *self.0.last().expect("never empty")
    }

    /// For screens that carry state, like a page number.
    pub fn top_mut(&mut self) -> &mut S {
        self.0.last_mut().expect("never empty")
    }

    pub fn push(&mut self, screen: S) {
        self.0.push(screen);
    }

    /// Pops one screen; does nothing at the root.
    pub fn pop(&mut self) {
        if self.0.len() > 1 {
            self.0.pop();
        }
    }

    /// Back to the root, discarding everything above it.
    pub fn reset(&mut self) {
        self.0.truncate(1);
    }

    pub fn iter(&self) -> impl Iterator<Item = &S> {
        self.0.iter()
    }
}

impl<S: Copy + PartialEq + Send + Sync + 'static> ScreenStack<S> {
    pub fn contains(&self, screen: S) -> bool {
        self.0.contains(&screen)
    }
}
