use std::path::PathBuf;

/// Get the trash/recycle bin path for the current OS
#[cfg(target_os = "linux")]
pub fn get_trash_path() -> Option<PathBuf> {
    // Try XDG standard trash location first
    dirs::home_dir().and_then(|home| {
        let xdg_trash = home.join(".local/share/Trash/files");
        if xdg_trash.exists() {
            Some(xdg_trash)
        } else {
            // Fallback to older .Trash directory
            let alt_trash = home.join(".Trash");
            if alt_trash.exists() {
                Some(alt_trash)
            } else {
                None
            }
        }
    })
}

#[cfg(target_os = "macos")]
pub fn get_trash_path() -> Option<PathBuf> {
    // macOS trash is always at ~/.Trash
    dirs::home_dir().map(|home| home.join(".Trash"))
}

#[cfg(target_os = "windows")]
pub fn get_trash_path() -> Option<PathBuf> {
    // Windows Recycle Bin location - typically C:\$Recycle.Bin
    // However, accessing it directly requires special permissions
    // For now, we return None (can be enhanced later with Windows API)
    None
}

/// Check if a given path is the trash directory
pub fn is_trash_path(path: &PathBuf) -> bool {
    if let Some(trash) = get_trash_path() {
        path == &trash || path.starts_with(&trash)
    } else {
        false
    }
}

/// Get human-readable name for the trash
pub fn get_trash_display_name() -> &'static str {
    #[cfg(target_os = "windows")]
    {
        "Recycle Bin"
    }

    #[cfg(not(target_os = "windows"))]
    {
        "Trash"
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trash_path_returns_option() {
        let trash = get_trash_path();
        // Doesn't panic, just returns Option
        let _ = trash;
    }

    #[test]
    fn test_is_trash_path() {
        if let Some(trash) = get_trash_path() {
            assert!(is_trash_path(&trash));
        }
    }
}
