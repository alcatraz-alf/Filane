use super::{FileManagerApp, ToastKind};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use std::time::Duration;

const RELEASE_API: &str = "https://api.github.com/repos/alcatraz-alf/Filane/releases/latest";

#[derive(Clone, Debug)]
pub(crate) struct UpdateInfo {
    pub tag: String,
    pub appimage_url: Option<String>,
    pub release_url: String,
    pub notes: String,
}

pub(crate) enum UpdateStatus {
    Idle,
    Checking,
    Done {
        outcome: Result<Option<UpdateInfo>, String>,
    },
}

impl FileManagerApp {
    /// Kick off a background update check. `quiet` suppresses user-visible
    /// failure/uptodate noise (used for the automatic startup check).
    pub(crate) fn start_update_check(&mut self, quiet: bool) {
        if matches!(self.update_status, UpdateStatus::Checking) {
            return;
        }
        self.update_status = UpdateStatus::Checking;
        self.update_check_quiet = quiet;
        self.last_update_check = Some(shell_now());
        self.update_result = Arc::new(Mutex::new(None));

        let shared = self.update_result.clone();
        let ctx = self.egui_ctx.clone();
        let current = env!("CARGO_PKG_VERSION").to_string();

        std::thread::spawn(move || {
            let outcome = fetch_latest_release(&current);
            if let Ok(mut slot) = shared.lock() {
                *slot = Some(outcome);
            }
            ctx.request_repaint();
        });
    }

    /// Drain a finished background check once it becomes available.
    pub(crate) fn poll_update_check(&mut self) {
        if !matches!(self.update_status, UpdateStatus::Checking) {
            return;
        }
        let outcome = self
            .update_result
            .lock()
            .ok()
            .and_then(|mut slot| slot.take());
        let Some(outcome) = outcome else { return };

        self.update_status = UpdateStatus::Done {
            outcome: outcome.clone(),
        };
        let quiet = self.update_check_quiet;

        match outcome {
            Ok(Some(info)) => {
                let headline: &str = info
                    .notes
                    .lines()
                    .find(|l| !l.trim().is_empty())
                    .unwrap_or("");
                self.push_toast(
                    if headline.is_empty() {
                        format!("🎉 Filane {} is available!", info.tag)
                    } else {
                        format!("🎉 Filane {} — {}", info.tag, headline)
                    },
                    ToastKind::Success,
                );
                let dl = info
                    .appimage_url
                    .as_deref()
                    .unwrap_or(info.release_url.as_str());
                self.status_message = format!("Update {} available — {}", info.tag, dl);
            }
            Ok(None) => {
                if !quiet {
                    self.status_message = format!(
                        "✅ Up to date (v{})",
                        env!("CARGO_PKG_VERSION")
                    );
                }
            }
            Err(e) => {
                if !quiet {
                    self.status_message = format!("⚠ Update check failed: {}", e);
                }
            }
        }
    }
}

/// Query the GitHub releases API and return `Some(info)` when the latest
/// release is newer than the running build.
fn fetch_latest_release(current: &str) -> Result<Option<UpdateInfo>, String> {
    let resp = ureq::get(RELEASE_API)
        .set("User-Agent", &format!("Filane/{}", current))
        .set("Accept", "application/vnd.github+json")
        .timeout(Duration::from_secs(10))
        .call()
        .map_err(|e| format!("network: {}", e))?;

    let json: Value = resp.into_json().map_err(|e| format!("parse: {}", e))?;
    let tag = json["tag_name"]
        .as_str()
        .ok_or("release has no tag_name")?
        .trim_start_matches('v')
        .to_string();

    if parse_version(&tag) <= parse_version(current) {
        return Ok(None);
    }

    let appimage_url = json["assets"]
        .as_array()
        .and_then(|assets| {
            assets
                .iter()
                .find(|a| {
                    a["browser_download_url"]
                        .as_str()
                        .map(|u| u.ends_with(".AppImage"))
                        .unwrap_or(false)
                })
                .and_then(|a| a["browser_download_url"].as_str().map(String::from))
        });

    let notes = json["body"]
        .as_str()
        .unwrap_or("")
        .chars()
        .take(400)
        .collect();

    Ok(Some(UpdateInfo {
        tag,
        appimage_url,
        release_url: json["html_url"]
            .as_str()
            .unwrap_or("https://github.com/alcatraz-alf/Filane/releases/latest")
            .to_string(),
        notes,
    }))
}

fn parse_version(s: &str) -> (u32, u32, u32) {
    let v: Vec<u32> = s
        .trim_start_matches('v')
        .split('.')
        .filter_map(|p| p.trim().parse().ok())
        .collect();
    (
        v.first().copied().unwrap_or(0),
        v.get(1).copied().unwrap_or(0),
        v.get(2).copied().unwrap_or(0),
    )
}

fn shell_now() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_comparison_handles_v_prefix_and_order() {
        assert!(parse_version("0.3.4") > parse_version("0.3.3"));
        assert!(parse_version("v1.0.0") > parse_version("0.9.9"));
        assert_eq!(parse_version("0.3.3"), parse_version("v0.3.3"));
        assert!(parse_version("0.11.0") > parse_version("0.10.1"));
        assert_eq!(parse_version("garbage").0, 0);
    }
}
