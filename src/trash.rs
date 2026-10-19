use anyhow::{bail, Context, Result};
use chrono::Local;
use std::fs;
use std::path::{Path, PathBuf};

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

/// Ensure the (XDG) trash directories exist and return (files_dir, info_dir).
#[cfg(any(target_os = "linux", target_os = "macos"))]
fn ensure_trash_dirs() -> Result<(PathBuf, PathBuf)> {
    let home = dirs::home_dir().context("Cannot determine home directory")?;
    // Use the XDG location, creating it if it doesn't exist yet
    let files_dir = home.join(".local/share/Trash/files");
    let info_dir = home.join(".local/share/Trash/info");
    fs::create_dir_all(&files_dir)
        .with_context(|| format!("Cannot create {}", files_dir.display()))?;
    fs::create_dir_all(&info_dir)
        .with_context(|| format!("Cannot create {}", info_dir.display()))?;
    Ok((files_dir, info_dir))
}

#[cfg(target_os = "windows")]
fn ensure_trash_dirs() -> Result<(PathBuf, PathBuf)> {
    bail!("Move to Trash is not supported on this platform yet")
}

/// Percent-encode characters that break the .trashinfo `Path=` field
/// (per the freedesktop.org Trash spec, only ASCII non-printables, '%' and
/// control chars must be encoded; we encode a pragmatic subset).
fn encode_trash_info_value(text: &str) -> String {
    let mut encoded = String::with_capacity(text.len());
    for byte in text.as_bytes() {
        match byte {
            b'%' | b'\n' | b'\r' | b' ' => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
            0x00..=0x1F | 0x7F if *byte != b' ' => {
                encoded.push_str(&format!("%{:02X}", byte));
            }
            _ => encoded.push(*byte as char),
        }
    }
    encoded
}

fn decode_trash_info_value(text: &str) -> String {
    let bytes: Vec<u8> = {
        let mut out = Vec::with_capacity(text.len());
        let ts = text.as_bytes();
        let mut i = 0;
        while i < ts.len() {
            if ts[i] == b'%' && i + 2 < ts.len() {
                if let Ok(v) = u8::from_str_radix(&text[i + 1..i + 3], 16) {
                    out.push(v);
                    i += 3;
                    continue;
                }
            }
            out.push(ts[i]);
            i += 1;
        }
        out
    };
    String::from_utf8_lossy(&bytes).into_owned()
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn write_trash_info(info_dir: &Path, unique_name: &str, original: &Path) -> Result<()> {
    let info_path = info_dir.join(format!("{}.trashinfo", unique_name));
    let contents = format!(
        "[Trash Info]\nPath={}\nDeletionDate={}",
        encode_trash_info_value(&original.display().to_string()),
        Local::now().format("%Y-%m-%dT%H:%M:%S")
    );
    fs::write(&info_path, contents)
        .with_context(|| format!("Cannot write {}", info_path.display()))
}

/// Recursive copy used as a fallback when rename fails with EXDEV.
fn copy_item_to(src: &Path, dst: &Path) -> Result<()> {
    if src.is_dir() {
        fs::create_dir_all(dst)
            .with_context(|| format!("Cannot create {}", dst.display()))?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let from = entry.path();
            let to = dst.join(entry.file_name());
            if from.is_dir() {
                copy_item_to(&from, &to)?;
            } else {
                fs::copy(&from, &to)
                    .with_context(|| format!("Cannot copy {}", from.display()))?;
            }
        }
    } else {
        fs::copy(src, dst).with_context(|| format!("Cannot copy {}", src.display()))?;
    }
    Ok(())
}

/// Move an item (file or directory) to the OS trash so it can be restored.
/// The trash directory is created if it doesn't exist. Name collisions in the
/// trash are resolved with a numeric suffix ("name.1").
pub fn move_to_trash(path: &Path) -> Result<PathBuf> {
    if path == Path::new("/") {
        bail!("Refusing to trash filesystem root");
    }

    let (files_dir, info_dir) = ensure_trash_dirs()?;

    let base_name = path
        .file_name()
        .context("Path has no filename")?
        .to_string_lossy()
        .to_string();
    if base_name == ".." || base_name == "." {
        bail!("Invalid path");
    }

    // Find a unique name inside the trash
    let mut unique_name = base_name.clone();
    let mut counter = 1;
    while files_dir.join(&unique_name).exists() || info_dir.join(format!("{}.trashinfo", unique_name)).exists()
    {
        unique_name = format!("{}.{}", base_name, counter);
        counter += 1;
    }

    let target = files_dir.join(&unique_name);
    match fs::rename(path, &target) {
        Ok(_) => {}
        // Cross-device (EXDEV): the source sits on another filesystem (e.g.
        // a USB drive), so rename cannot reach the home trash. Copy the
        // item, then remove the original.
        Err(e) if e.raw_os_error() == Some(18) => {
            copy_item_to(path, &target)?;
            if path.is_dir() {
                fs::remove_dir_all(path)
                    .with_context(|| format!("Cannot remove {} after copy", path.display()))?;
            } else {
                fs::remove_file(path)
                    .with_context(|| format!("Cannot remove {} after copy", path.display()))?;
            }
        }
        Err(e) => {
            return Err(e)
                .with_context(|| format!("Cannot move {} to trash", path.display()));
        }
    };

    // Record the original location so we can restore later
    write_trash_info(&info_dir, &unique_name, path)?;

    Ok(target)
}

/// Restore an item from the trash back to its original location
/// (decoded from its .trashinfo file). Returns the restored path.
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub fn restore_from_trash(item_path: &Path) -> Result<PathBuf> {
    let files_dir = item_path
        .parent()
        .context("Item has no parent")?
        .to_path_buf();
    let info_dir = files_dir
        .parent()
        .context("Trash has no parent")?
        .join("info");

    let name = item_path
        .file_name()
        .context("Item has no filename")?
        .to_string_lossy()
        .to_string();

    let info_path = info_dir.join(format!("{}.trashinfo", name));
    let info_contents = fs::read_to_string(&info_path)
        .with_context(|| format!("No trash info found for '{}'", name))?;

    let mut original_path: Option<PathBuf> = None;
    for line in info_contents.lines() {
        if let Some(value) = line.strip_prefix("Path=") {
            original_path = Some(PathBuf::from(decode_trash_info_value(value)));
        }
    }
    let original =
        original_path.context("Trash info is missing the original 'Path' entry")?;

    // Refuse to clobber an existing item at the original location
    let mut target = original.clone();
    let mut counter = 1;
    while target.exists() {
        let file_stem = original
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or(&name);
        let ext = original.extension().and_then(|e| e.to_str());
        target = match ext {
            Some(ext) => original.with_file_name(format!("{} (restored {}).{}", file_stem, counter, ext)),
            None => original.with_file_name(format!("{} (restored {})", file_stem, counter)),
        };
        // `.with_file_name` on an empty stem would loop forever; bail out
        if counter > 999 {
            bail!("Could not find a free name next to {}", original.display());
        }
        counter += 1;
    }    match fs::rename(item_path, &target) {
        Ok(_) => {}
        // Cross-device (EXDEV): the original location sits on another
        // filesystem. Copy the item back, then remove the trash copy.
        Err(e) if e.raw_os_error() == Some(18) => {
            copy_item_to(item_path, &target)?;
            if item_path.is_dir() {
                fs::remove_dir_all(item_path)
                    .with_context(|| format!("Cannot clear {}", item_path.display()))?;
            } else {
                fs::remove_file(item_path)
                    .with_context(|| format!("Cannot clear {}", item_path.display()))?;
            }
        }
        Err(e) => {
            return Err(e)
                .with_context(|| format!("Cannot restore {}", item_path.display()));
        }
    }

    // Cleanup: trash metadata is no longer needed
    let _ = fs::remove_file(&info_path);

    Ok(target)
}

#[cfg(target_os = "windows")]
pub fn restore_from_trash(_item_path: &Path) -> Result<PathBuf> {
    bail!("Restore from Trash is not supported on this platform yet")
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

    #[test]
    fn test_trash_info_encoding_roundtrip() {
        let original = "/home/user/my folder 100%/file.txt";
        let encoded = encode_trash_info_value(original);
        assert!(encoded.contains("%20"));
        assert!(!encoded.contains(' '));
        assert_eq!(decode_trash_info_value(&encoded), original);
    }

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    #[test]
    fn test_move_to_trash_and_restore() {
        let temp = std::env::temp_dir().join(format!(
            "fm_trash_test_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .subsec_nanos()
        ));
        fs::create_dir_all(&temp).unwrap();
        let file = temp.join("sample.txt");
        fs::write(&file, b"hello trash").unwrap();

        let trashed = move_to_trash(&file).expect("move to trash");
        assert!(!file.exists());
        assert!(trashed.exists());

        let restored = restore_from_trash(&trashed).expect("restore");
        assert!(restored.exists());
        assert_eq!(fs::read(&restored).unwrap(), b"hello trash");

        let _ = fs::remove_dir_all(&temp);
    }
}
