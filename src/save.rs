//! RON files under a `SaveDir` resource. It is a resource so tests can point it at
//! a temp dir; the default is the platform data directory.

use std::env;
use std::path::PathBuf;

use bevy::prelude::Resource;
use serde::Serialize;
use serde::de::DeserializeOwned;

#[derive(Resource, Clone, Debug)]
pub struct SaveDir(pub PathBuf);

impl SaveDir {
    /// `%APPDATA%/<game>` on Windows, `$XDG_DATA_HOME/<game>` or `$HOME/<game>` elsewhere.
    pub fn for_game(name: &str) -> Self {
        let base = env::var_os("APPDATA")
            .or_else(|| env::var_os("XDG_DATA_HOME"))
            .or_else(|| env::var_os("HOME"))
            .map(PathBuf::from)
            .unwrap_or_else(|| PathBuf::from("."));
        SaveDir(base.join(name))
    }

    /// A fresh temp dir for tests, unique per process and tag.
    pub fn temp(tag: &str) -> Self {
        let dir = env::temp_dir().join(format!("{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        SaveDir(dir)
    }

    pub fn path(&self, name: &str) -> PathBuf {
        self.0.join(format!("{name}.ron"))
    }

    pub fn exists(&self, name: &str) -> bool {
        self.path(name).is_file()
    }

    pub fn save<T: Serialize>(&self, name: &str, value: &T) -> Result<(), String> {
        let text =
            ron::ser::to_string_pretty(value, Default::default()).map_err(|e| e.to_string())?;
        std::fs::create_dir_all(&self.0).map_err(|e| e.to_string())?;
        std::fs::write(self.path(name), text).map_err(|e| e.to_string())
    }

    pub fn load<T: DeserializeOwned>(&self, name: &str) -> Result<T, String> {
        let text = std::fs::read_to_string(self.path(name)).map_err(|e| e.to_string())?;
        ron::from_str(&text).map_err(|e| e.to_string())
    }

    pub fn delete(&self, name: &str) {
        let _ = std::fs::remove_file(self.path(name));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip() {
        let dir = SaveDir::temp("bevy-term-save");
        assert!(!dir.exists("g"));
        dir.save("g", &(1u32, "x".to_string())).unwrap();
        assert!(dir.exists("g"));
        let back: (u32, String) = dir.load("g").unwrap();
        assert_eq!(back, (1, "x".to_string()));
        dir.delete("g");
        assert!(!dir.exists("g"));
    }
}
