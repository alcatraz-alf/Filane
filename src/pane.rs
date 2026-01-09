use crate::filesystem::{FileItem, read_directory};
use anyhow::Result;
use std::path::PathBuf;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortBy {
    Name,
    Size,
    Date,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SortOrder {
    Ascending,
    Descending,
}

pub struct Pane {
    pub current_path: PathBuf,
    pub items: Vec<FileItem>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
    pub filter_text: String,
    pub history: Vec<PathBuf>,
    pub history_index: usize,
    pub git_repo_path: Option<PathBuf>,
    pub selected_items: Vec<usize>,
    pub selection_anchor: Option<usize>,
}

impl Pane {
    pub fn new(path: PathBuf) -> Result<Self> {
        let git_repo_path = crate::filesystem::find_git_repo(&path);

        let mut pane = Pane {
            current_path: path.clone(),
            items: Vec::new(),
            selected_index: 0,
            scroll_offset: 0,
            sort_by: SortBy::Name,
            sort_order: SortOrder::Ascending,
            filter_text: String::new(),
            history: vec![path],
            history_index: 0,
            git_repo_path,
            selected_items: Vec::new(),
            selection_anchor: None,
        };
        pane.refresh()?;
        Ok(pane)
    }

    pub fn refresh(&mut self) -> Result<()> {
        self.items = read_directory(&self.current_path)?;

        if let Some(repo_path) = &self.git_repo_path {
            crate::filesystem::apply_git_status(&mut self.items, repo_path);
        }

        self.apply_sort();
        if self.selected_index >= self.items.len() && !self.items.is_empty() {
            self.selected_index = self.items.len() - 1;
        }
        Ok(())
    }

    pub fn toggle_sort(&mut self, sort_by: SortBy) {
        if self.sort_by == sort_by {
            // Toggle order if clicking same column
            self.sort_order = match self.sort_order {
                SortOrder::Ascending => SortOrder::Descending,
                SortOrder::Descending => SortOrder::Ascending,
            };
        } else {
            self.sort_by = sort_by;
            self.sort_order = SortOrder::Ascending;
        }
        self.apply_sort();
    }

    fn apply_sort(&mut self) {
        // Keep ".." at the top
        let parent = self.items.iter().position(|item| item.name == "..");
        if let Some(idx) = parent {
            let parent_item = self.items.remove(idx);

            // Sort remaining items
            self.items.sort_by(|a, b| {
                let ordering = match self.sort_by {
                    SortBy::Name => {
                        // Directories first, then by name
                        match (a.is_dir, b.is_dir) {
                            (true, false) => std::cmp::Ordering::Less,
                            (false, true) => std::cmp::Ordering::Greater,
                            _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
                        }
                    }
                    SortBy::Size => a.size.cmp(&b.size),
                    SortBy::Date => a.modified.cmp(&b.modified),
                };

                match self.sort_order {
                    SortOrder::Ascending => ordering,
                    SortOrder::Descending => ordering.reverse(),
                }
            });

            // Re-insert parent at the beginning
            self.items.insert(0, parent_item);
        }
    }

    pub fn move_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_down(&mut self) {
        if self.selected_index < self.items.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    pub fn move_up_with_selection(&mut self) {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.selected_index);
        }
        
        if self.selected_index > 0 {
            self.selected_index -= 1;
            self.update_selection_range();
        }
    }

    pub fn move_down_with_selection(&mut self) {
        if self.selection_anchor.is_none() {
            self.selection_anchor = Some(self.selected_index);
        }
        
        if self.selected_index < self.items.len().saturating_sub(1) {
            self.selected_index += 1;
            self.update_selection_range();
        }
    }

    fn update_selection_range(&mut self) {
        if let Some(anchor) = self.selection_anchor {
            self.selected_items.clear();
            let start = anchor.min(self.selected_index);
            let end = anchor.max(self.selected_index);
            self.selected_items = (start..=end).collect();
        }
    }

    pub fn clear_selection(&mut self) {
        self.selected_items.clear();
        self.selection_anchor = None;
    }

    pub fn is_item_selected(&self, index: usize) -> bool {
        self.selected_items.contains(&index)
    }

    pub fn get_selected_items(&self) -> Vec<&FileItem> {
        if self.selected_items.is_empty() {
            if let Some(item) = self.items.get(self.selected_index) {
                vec![item]
            } else {
                Vec::new()
            }
        } else {
            self.selected_items
                .iter()
                .filter_map(|&idx| self.items.get(idx))
                .collect()
        }
    }

    pub fn enter_directory(&mut self) -> Result<()> {
        if let Some(item) = self.items.get(self.selected_index) {
            if item.is_dir {
                let new_path = if item.name == ".." {
                    self.current_path
                        .parent()
                        .unwrap_or(&self.current_path)
                        .to_path_buf()
                } else {
                    item.path.clone()
                };

                self.navigate_to(new_path.canonicalize()?)?;
            }
        }
        Ok(())
    }

    pub fn navigate_to(&mut self, path: PathBuf) -> Result<()> {
        self.current_path = path.clone();

        self.git_repo_path = crate::filesystem::find_git_repo(&path);

        self.refresh()?;
        self.selected_index = 0;
        self.scroll_offset = 0;

        // Add to history
        // Remove any forward history if we're not at the end
        if self.history_index < self.history.len() - 1 {
            self.history.truncate(self.history_index + 1);
        }

        // Don't add if it's the same as current
        if self.history.last() != Some(&path) {
            self.history.push(path);
            self.history_index = self.history.len() - 1;

            // Limit history to 50 items
            if self.history.len() > 50 {
                self.history.remove(0);
                self.history_index = self.history.len() - 1;
            }
        }

        Ok(())
    }

    pub fn navigate_back(&mut self) -> Result<()> {
        if self.can_go_back() {
            self.history_index -= 1;
            self.current_path = self.history[self.history_index].clone();
            self.refresh()?;
            self.selected_index = 0;
            self.scroll_offset = 0;
        }
        Ok(())
    }

    pub fn navigate_forward(&mut self) -> Result<()> {
        if self.can_go_forward() {
            self.history_index += 1;
            self.current_path = self.history[self.history_index].clone();
            self.refresh()?;
            self.selected_index = 0;
            self.scroll_offset = 0;
        }
        Ok(())
    }

    pub fn can_go_back(&self) -> bool {
        self.history_index > 0
    }

    pub fn can_go_forward(&self) -> bool {
        self.history_index < self.history.len().saturating_sub(1)
    }

    pub fn get_selected_item(&self) -> Option<&FileItem> {
        self.items.get(self.selected_index)
    }

    pub fn update_scroll(&mut self, viewport_height: usize) {
        if self.selected_index < self.scroll_offset {
            self.scroll_offset = self.selected_index;
        } else if self.selected_index >= self.scroll_offset + viewport_height {
            self.scroll_offset = self.selected_index - viewport_height + 1;
        }
    }
}
