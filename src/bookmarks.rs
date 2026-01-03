use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Bookmark {
    pub name: String,
    pub path: PathBuf,
    pub icon: String,
}

impl Bookmark {
    pub fn new(name: String, path: PathBuf, icon: String) -> Self {
        Self { name, path, icon }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BookmarkManager {
    pub bookmarks: Vec<Bookmark>,
}

impl BookmarkManager {
    pub fn new() -> Self {
        Self {
            bookmarks: Vec::new(),
        }
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            let manager: BookmarkManager = serde_json::from_str(&content)?;
            Ok(manager)
        } else {
            Ok(Self::new())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::get_config_path()?;

        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;

        Ok(())
    }

    pub fn add_bookmark(&mut self, name: String, path: PathBuf, icon: String) -> Result<()> {
        if !self.bookmarks.iter().any(|b| b.path == path) {
            self.bookmarks.push(Bookmark::new(name, path, icon));
            self.save()?;
        }
        Ok(())
    }

    pub fn remove_bookmark(&mut self, index: usize) -> Result<()> {
        if index < self.bookmarks.len() {
            self.bookmarks.remove(index);
            self.save()?;
        }
        Ok(())
    }

    pub fn get_bookmarks(&self) -> &[Bookmark] {
        &self.bookmarks
    }

    fn get_config_path() -> Result<PathBuf> {
        let config_dir =
            dirs::config_dir().ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("dual-pane-fm").join("bookmarks.json"))
    }
}

impl Default for BookmarkManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn get_quick_access_items() -> Vec<Bookmark> {
    let mut items = Vec::new();

    if let Some(home) = dirs::home_dir() {
        items.push(Bookmark::new(
            "Home".to_string(),
            home.clone(),
            "🏠".to_string(),
        ));

        items.push(Bookmark::new(
            "Documents".to_string(),
            home.join("Documents"),
            "📄".to_string(),
        ));
        items.push(Bookmark::new(
            "Downloads".to_string(),
            home.join("Downloads"),
            "📥".to_string(),
        ));
        items.push(Bookmark::new(
            "Pictures".to_string(),
            home.join("Pictures"),
            "🖼️".to_string(),
        ));
        items.push(Bookmark::new(
            "Music".to_string(),
            home.join("Music"),
            "🎵".to_string(),
        ));
        items.push(Bookmark::new(
            "Videos".to_string(),
            home.join("Videos"),
            "🎬".to_string(),
        ));
        items.push(Bookmark::new(
            "Desktop".to_string(),
            home.join("Desktop"),
            "🖥️".to_string(),
        ));

        // Add Trash if available
        if let Some(trash_path) = crate::trash::get_trash_path() {
            items.push(Bookmark::new(
                crate::trash::get_trash_display_name().to_string(),
                trash_path,
                "🗑".to_string(),
            ));
        }
    }

    items.retain(|item| item.path.exists());

    items
}
