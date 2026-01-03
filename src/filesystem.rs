use anyhow::Result;
use std::collections::HashMap;
use std::fs::{self, DirEntry};
use std::io::{BufRead, BufReader, Read};
use std::path::{Path, PathBuf};
use std::time::SystemTime;
use sysinfo::{DiskKind, Disks};

#[derive(Clone, Debug, PartialEq)]
pub enum GitStatus {
    Unmodified,
    Modified,
    Added,
    Deleted,
    Renamed,
    Copied,
    Untracked,
    Ignored,
}

impl GitStatus {
    pub fn icon(&self) -> &str {
        match self {
            GitStatus::Unmodified => "",
            GitStatus::Modified => "M",
            GitStatus::Added => "A",
            GitStatus::Deleted => "D",
            GitStatus::Renamed => "R",
            GitStatus::Copied => "C",
            GitStatus::Untracked => "?",
            GitStatus::Ignored => "!",
        }
    }

    pub fn color(&self) -> egui::Color32 {
        match self {
            GitStatus::Unmodified => egui::Color32::from_rgb(154, 160, 166),
            GitStatus::Modified => egui::Color32::from_rgb(255, 193, 7),
            GitStatus::Added => egui::Color32::from_rgb(129, 201, 149),
            GitStatus::Deleted => egui::Color32::from_rgb(242, 139, 130),
            GitStatus::Renamed => egui::Color32::from_rgb(138, 180, 248),
            GitStatus::Copied => egui::Color32::from_rgb(138, 180, 248),
            GitStatus::Untracked => egui::Color32::from_rgb(189, 147, 249),
            GitStatus::Ignored => egui::Color32::from_rgb(100, 100, 100),
        }
    }
}

#[derive(Clone, Debug)]
pub struct FileItem {
    pub name: String,
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: SystemTime,
    pub git_status: Option<GitStatus>,
}

#[derive(Clone, Debug)]
pub struct DirectoryStats {
    pub total_items: usize,
    pub folder_count: usize,
    pub file_count: usize,
    pub total_size: u64,
}

impl FileItem {
    pub fn from_entry(entry: &DirEntry) -> Result<Self> {
        let metadata = entry.metadata()?;
        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        Ok(FileItem {
            name: entry.file_name().to_string_lossy().to_string(),
            path: entry.path(),
            is_dir: metadata.is_dir(),
            size: metadata.len(),
            modified,
            git_status: None,
        })
    }

    pub fn parent_dir() -> Self {
        FileItem {
            name: "..".to_string(),
            path: PathBuf::from(".."),
            is_dir: true,
            size: 0,
            modified: SystemTime::UNIX_EPOCH,
            git_status: None,
        }
    }
}

pub fn read_directory(path: &Path) -> Result<Vec<FileItem>> {
    let mut items = vec![FileItem::parent_dir()];

    let entries = fs::read_dir(path)?;
    let mut file_items: Vec<FileItem> = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| FileItem::from_entry(&entry).ok())
        .collect();

    // Sort: directories first, then files, alphabetically
    file_items.sort_by(|a, b| match (a.is_dir, b.is_dir) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.name.to_lowercase().cmp(&b.name.to_lowercase()),
    });

    items.extend(file_items);
    Ok(items)
}

pub fn format_size(size: u64) -> String {
    const KB: u64 = 1024;
    const MB: u64 = KB * 1024;
    const GB: u64 = MB * 1024;

    if size >= GB {
        format!("{:.2} GB", size as f64 / GB as f64)
    } else if size >= MB {
        format!("{:.2} MB", size as f64 / MB as f64)
    } else if size >= KB {
        format!("{:.2} KB", size as f64 / KB as f64)
    } else {
        format!("{} B", size)
    }
}

pub fn format_date(time: SystemTime) -> String {
    use chrono::{DateTime, Local};
    let datetime: DateTime<Local> = time.into();
    datetime.format("%Y-%m-%d %H:%M").to_string()
}

pub fn get_permissions(path: &Path) -> Result<(String, String)> {
    let metadata = fs::metadata(path)?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mode = metadata.permissions().mode();

        let user = format!(
            "{}{}{}",
            if mode & 0o400 != 0 { "r" } else { "-" },
            if mode & 0o200 != 0 { "w" } else { "-" },
            if mode & 0o100 != 0 { "x" } else { "-" }
        );
        let group = format!(
            "{}{}{}",
            if mode & 0o040 != 0 { "r" } else { "-" },
            if mode & 0o020 != 0 { "w" } else { "-" },
            if mode & 0o010 != 0 { "x" } else { "-" }
        );
        let others = format!(
            "{}{}{}",
            if mode & 0o004 != 0 { "r" } else { "-" },
            if mode & 0o002 != 0 { "w" } else { "-" },
            if mode & 0o001 != 0 { "x" } else { "-" }
        );

        let symbolic = format!("{}{}{}", user, group, others);
        let octal = format!("{:o}", mode & 0o777);

        let mut descriptions = Vec::new();

        if mode & 0o400 != 0 || mode & 0o200 != 0 || mode & 0o100 != 0 {
            let mut owner_perms = Vec::new();
            if mode & 0o400 != 0 {
                owner_perms.push("Read");
            }
            if mode & 0o200 != 0 {
                owner_perms.push("Write");
            }
            if mode & 0o100 != 0 {
                owner_perms.push("Execute");
            }
            descriptions.push(format!("Owner: {}", owner_perms.join(", ")));
        } else {
            descriptions.push("Owner: No access".to_string());
        }

        if mode & 0o040 != 0 || mode & 0o020 != 0 || mode & 0o010 != 0 {
            let mut group_perms = Vec::new();
            if mode & 0o040 != 0 {
                group_perms.push("Read");
            }
            if mode & 0o020 != 0 {
                group_perms.push("Write");
            }
            if mode & 0o010 != 0 {
                group_perms.push("Execute");
            }
            descriptions.push(format!("Group: {}", group_perms.join(", ")));
        } else {
            descriptions.push("Group: No access".to_string());
        }

        if mode & 0o004 != 0 || mode & 0o002 != 0 || mode & 0o001 != 0 {
            let mut other_perms = Vec::new();
            if mode & 0o004 != 0 {
                other_perms.push("Read");
            }
            if mode & 0o002 != 0 {
                other_perms.push("Write");
            }
            if mode & 0o001 != 0 {
                other_perms.push("Execute");
            }
            descriptions.push(format!("Others: {}", other_perms.join(", ")));
        } else {
            descriptions.push("Others: No access".to_string());
        }

        let summary = format!("{} ({})", symbolic, octal);
        let details = descriptions.join("\n");

        Ok((summary, details))
    }

    #[cfg(windows)]
    {
        let readonly = metadata.permissions().readonly();
        if readonly {
            Ok((
                "Read-only".to_string(),
                "File can only be read, not modified".to_string(),
            ))
        } else {
            Ok((
                "Read/Write".to_string(),
                "File can be read and modified".to_string(),
            ))
        }
    }
}

pub fn calculate_directory_stats(items: &[FileItem]) -> DirectoryStats {
    let mut folder_count = 0;
    let mut file_count = 0;
    let mut total_size = 0u64;

    for item in items {
        if item.name == ".." {
            continue;
        }

        if item.is_dir {
            folder_count += 1;
        } else {
            file_count += 1;
            total_size += item.size;
        }
    }

    DirectoryStats {
        total_items: folder_count + file_count,
        folder_count,
        file_count,
        total_size,
    }
}

pub fn is_archive(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_lower = ext.to_string_lossy().to_lowercase();
        matches!(
            ext_lower.as_str(),
            "zip" | "tar" | "gz" | "tgz" | "bz2" | "xz" | "7z" | "rar"
        )
    } else {
        false
    }
}

pub fn compress_to_zip(source_path: &Path, dest_zip: &Path) -> Result<()> {
    use std::io::Write;

    let file = std::fs::File::create(dest_zip)?;
    let mut zip = zip::ZipWriter::new(file);

    let options = zip::write::FileOptions::default()
        .compression_method(zip::CompressionMethod::Deflated)
        .unix_permissions(0o755);

    if source_path.is_file() {
        let name = source_path
            .file_name()
            .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
            .to_string_lossy();

        zip.start_file(name.as_ref(), options)?;
        let content = std::fs::read(source_path)?;
        zip.write_all(&content)?;
    } else if source_path.is_dir() {
        add_directory_to_zip(&mut zip, source_path, source_path, options)?;
    }

    zip.finish()?;
    Ok(())
}

fn add_directory_to_zip(
    zip: &mut zip::ZipWriter<std::fs::File>,
    base_path: &Path,
    current_path: &Path,
    options: zip::write::FileOptions,
) -> Result<()> {
    use std::io::Write;

    for entry in std::fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(base_path)?;

        if path.is_file() {
            zip.start_file(name.to_string_lossy().as_ref(), options)?;
            let content = std::fs::read(&path)?;
            zip.write_all(&content)?;
        } else if path.is_dir() {
            zip.add_directory(name.to_string_lossy().as_ref(), options)?;
            add_directory_to_zip(zip, base_path, &path, options)?;
        }
    }

    Ok(())
}

pub fn extract_zip(zip_path: &Path, dest_dir: &Path) -> Result<()> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = zip::ZipArchive::new(file)?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = dest_dir.join(file.name());

        if file.name().ends_with('/') {
            std::fs::create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    std::fs::create_dir_all(p)?;
                }
            }
            let mut outfile = std::fs::File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if let Some(mode) = file.unix_mode() {
                std::fs::set_permissions(&outpath, std::fs::Permissions::from_mode(mode))?;
            }
        }
    }

    Ok(())
}

#[derive(Clone, Debug)]
pub struct SearchCriteria {
    pub search_path: PathBuf,
    pub filename_pattern: String,
    pub content_pattern: String,
    pub min_size: Option<u64>,
    pub max_size: Option<u64>,
    pub modified_after: Option<SystemTime>,
    pub modified_before: Option<SystemTime>,
    pub file_type: SearchFileType,
    pub case_sensitive: bool,
    pub include_hidden: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SearchFileType {
    All,
    Files,
    Directories,
}

impl Default for SearchCriteria {
    fn default() -> Self {
        Self {
            search_path: PathBuf::from("."),
            filename_pattern: String::new(),
            content_pattern: String::new(),
            min_size: None,
            max_size: None,
            modified_after: None,
            modified_before: None,
            file_type: SearchFileType::All,
            case_sensitive: false,
            include_hidden: false,
        }
    }
}

pub fn search_files(criteria: &SearchCriteria) -> Result<Vec<FileItem>> {
    let mut results = Vec::new();
    search_recursive(&criteria.search_path, criteria, &mut results)?;
    Ok(results)
}

fn search_recursive(
    path: &Path,
    criteria: &SearchCriteria,
    results: &mut Vec<FileItem>,
) -> Result<()> {
    if !path.is_dir() {
        return Ok(());
    }

    let entries = match fs::read_dir(path) {
        Ok(e) => e,
        Err(_) => return Ok(()),
    };

    for entry in entries.flatten() {
        let entry_path = entry.path();
        let file_name = entry.file_name().to_string_lossy().to_string();

        if !criteria.include_hidden && file_name.starts_with('.') {
            continue;
        }

        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(_) => continue,
        };

        let is_dir = metadata.is_dir();

        match criteria.file_type {
            SearchFileType::Files if is_dir => {
                if is_dir {
                    let _ = search_recursive(&entry_path, criteria, results);
                }
                continue;
            }
            SearchFileType::Directories if !is_dir => continue,
            _ => {}
        }

        if !criteria.filename_pattern.is_empty() {
            let matches = if criteria.case_sensitive {
                file_name.contains(&criteria.filename_pattern)
            } else {
                file_name
                    .to_lowercase()
                    .contains(&criteria.filename_pattern.to_lowercase())
            };

            if !matches {
                if is_dir {
                    let _ = search_recursive(&entry_path, criteria, results);
                }
                continue;
            }
        }

        let size = metadata.len();
        if let Some(min) = criteria.min_size {
            if size < min {
                if is_dir {
                    let _ = search_recursive(&entry_path, criteria, results);
                }
                continue;
            }
        }
        if let Some(max) = criteria.max_size {
            if size > max {
                if is_dir {
                    let _ = search_recursive(&entry_path, criteria, results);
                }
                continue;
            }
        }

        let modified = metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH);
        if let Some(after) = criteria.modified_after {
            if modified < after {
                if is_dir {
                    let _ = search_recursive(&entry_path, criteria, results);
                }
                continue;
            }
        }
        if let Some(before) = criteria.modified_before {
            if modified > before {
                if is_dir {
                    let _ = search_recursive(&entry_path, criteria, results);
                }
                continue;
            }
        }

        if !criteria.content_pattern.is_empty() && !is_dir {
            let content_matches = search_file_content(
                &entry_path,
                &criteria.content_pattern,
                criteria.case_sensitive,
            );
            if !content_matches {
                continue;
            }
        }

        results.push(FileItem {
            name: file_name,
            path: entry_path.clone(),
            is_dir,
            size,
            modified,
            git_status: None,
        });

        if is_dir {
            let _ = search_recursive(&entry_path, criteria, results);
        }
    }

    Ok(())
}

fn search_file_content(path: &Path, pattern: &str, case_sensitive: bool) -> bool {
    let mut file = match fs::File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut content = String::new();
    if file.read_to_string(&mut content).is_err() {
        return false;
    }

    if case_sensitive {
        content.contains(pattern)
    } else {
        content.to_lowercase().contains(&pattern.to_lowercase())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum DiffLineType {
    Equal,
    Added,
    Removed,
    Modified,
}

#[derive(Clone, Debug)]
pub struct DiffLine {
    pub line_type: DiffLineType,
    pub left_line_num: Option<usize>,
    pub right_line_num: Option<usize>,
    pub left_content: String,
    pub right_content: String,
}

#[derive(Clone, Debug)]
pub struct FileComparison {
    pub left_path: PathBuf,
    pub right_path: PathBuf,
    pub are_identical: bool,
    pub diff_lines: Vec<DiffLine>,
    pub left_only_lines: usize,
    pub right_only_lines: usize,
    pub modified_lines: usize,
    pub equal_lines: usize,
}

pub fn compare_files(left_path: &Path, right_path: &Path) -> Result<FileComparison> {
    let left_metadata = fs::metadata(left_path)?;
    let right_metadata = fs::metadata(right_path)?;

    if left_metadata.is_dir() || right_metadata.is_dir() {
        return Err(anyhow::anyhow!("Cannot compare directories"));
    }

    if left_metadata.len() != right_metadata.len() {
        return compare_text_files(left_path, right_path);
    }

    let left_content = fs::read(left_path)?;
    let right_content = fs::read(right_path)?;

    if left_content == right_content {
        return Ok(FileComparison {
            left_path: left_path.to_path_buf(),
            right_path: right_path.to_path_buf(),
            are_identical: true,
            diff_lines: vec![],
            left_only_lines: 0,
            right_only_lines: 0,
            modified_lines: 0,
            equal_lines: 0,
        });
    }

    compare_text_files(left_path, right_path)
}

fn compare_text_files(left_path: &Path, right_path: &Path) -> Result<FileComparison> {
    let left_file = fs::File::open(left_path)?;
    let right_file = fs::File::open(right_path)?;

    let left_reader = BufReader::new(left_file);
    let right_reader = BufReader::new(right_file);

    let left_lines: Vec<String> = left_reader.lines().filter_map(|l| l.ok()).collect();
    let right_lines: Vec<String> = right_reader.lines().filter_map(|l| l.ok()).collect();

    let diff_lines = compute_diff(&left_lines, &right_lines);

    let mut left_only = 0;
    let mut right_only = 0;
    let mut modified = 0;
    let mut equal = 0;

    for line in &diff_lines {
        match line.line_type {
            DiffLineType::Equal => equal += 1,
            DiffLineType::Added => right_only += 1,
            DiffLineType::Removed => left_only += 1,
            DiffLineType::Modified => modified += 1,
        }
    }

    let are_identical = left_only == 0 && right_only == 0 && modified == 0;

    Ok(FileComparison {
        left_path: left_path.to_path_buf(),
        right_path: right_path.to_path_buf(),
        are_identical,
        diff_lines,
        left_only_lines: left_only,
        right_only_lines: right_only,
        modified_lines: modified,
        equal_lines: equal,
    })
}

fn compute_diff(left_lines: &[String], right_lines: &[String]) -> Vec<DiffLine> {
    let mut result = Vec::new();
    let mut left_idx = 0;
    let mut right_idx = 0;

    while left_idx < left_lines.len() || right_idx < right_lines.len() {
        if left_idx >= left_lines.len() {
            result.push(DiffLine {
                line_type: DiffLineType::Added,
                left_line_num: None,
                right_line_num: Some(right_idx + 1),
                left_content: String::new(),
                right_content: right_lines[right_idx].clone(),
            });
            right_idx += 1;
        } else if right_idx >= right_lines.len() {
            result.push(DiffLine {
                line_type: DiffLineType::Removed,
                left_line_num: Some(left_idx + 1),
                right_line_num: None,
                left_content: left_lines[left_idx].clone(),
                right_content: String::new(),
            });
            left_idx += 1;
        } else if left_lines[left_idx] == right_lines[right_idx] {
            result.push(DiffLine {
                line_type: DiffLineType::Equal,
                left_line_num: Some(left_idx + 1),
                right_line_num: Some(right_idx + 1),
                left_content: left_lines[left_idx].clone(),
                right_content: right_lines[right_idx].clone(),
            });
            left_idx += 1;
            right_idx += 1;
        } else {
            let look_ahead = 5;
            let mut found_match = false;

            for i in 1..=look_ahead {
                if left_idx + i < left_lines.len()
                    && left_lines[left_idx + i] == right_lines[right_idx]
                {
                    for _ in 0..i {
                        result.push(DiffLine {
                            line_type: DiffLineType::Removed,
                            left_line_num: Some(left_idx + 1),
                            right_line_num: None,
                            left_content: left_lines[left_idx].clone(),
                            right_content: String::new(),
                        });
                        left_idx += 1;
                    }
                    found_match = true;
                    break;
                }

                if right_idx + i < right_lines.len()
                    && left_lines[left_idx] == right_lines[right_idx + i]
                {
                    for _ in 0..i {
                        result.push(DiffLine {
                            line_type: DiffLineType::Added,
                            left_line_num: None,
                            right_line_num: Some(right_idx + 1),
                            left_content: String::new(),
                            right_content: right_lines[right_idx].clone(),
                        });
                        right_idx += 1;
                    }
                    found_match = true;
                    break;
                }
            }

            if !found_match {
                result.push(DiffLine {
                    line_type: DiffLineType::Modified,
                    left_line_num: Some(left_idx + 1),
                    right_line_num: Some(right_idx + 1),
                    left_content: left_lines[left_idx].clone(),
                    right_content: right_lines[right_idx].clone(),
                });
                left_idx += 1;
                right_idx += 1;
            }
        }
    }

    result
}

#[derive(Clone, Debug)]
pub struct GitRepoInfo {
    pub current_branch: String,
    pub ahead: usize,
    pub behind: usize,
    pub has_changes: bool,
}

pub fn find_git_repo(path: &Path) -> Option<PathBuf> {
    let mut current = path;
    loop {
        let git_dir = current.join(".git");
        if git_dir.exists() {
            return Some(current.to_path_buf());
        }

        current = current.parent()?;
    }
}

pub fn get_git_status(repo_path: &Path) -> Result<HashMap<PathBuf, GitStatus>> {
    let repo = git2::Repository::open(repo_path)?;
    let mut status_map = HashMap::new();

    let statuses = repo.statuses(Some(
        git2::StatusOptions::new()
            .include_untracked(true)
            .recurse_untracked_dirs(true),
    ))?;

    for entry in statuses.iter() {
        let path = entry.path().unwrap_or("");
        let full_path = repo_path.join(path);

        let status = match entry.status() {
            s if s.contains(git2::Status::WT_NEW) => GitStatus::Untracked,
            s if s.contains(git2::Status::WT_MODIFIED) => GitStatus::Modified,
            s if s.contains(git2::Status::WT_DELETED) => GitStatus::Deleted,
            s if s.contains(git2::Status::WT_RENAMED) => GitStatus::Renamed,
            s if s.contains(git2::Status::INDEX_NEW) => GitStatus::Added,
            s if s.contains(git2::Status::INDEX_MODIFIED) => GitStatus::Modified,
            s if s.contains(git2::Status::INDEX_DELETED) => GitStatus::Deleted,
            s if s.contains(git2::Status::INDEX_RENAMED) => GitStatus::Renamed,
            s if s.contains(git2::Status::IGNORED) => GitStatus::Ignored,
            _ => GitStatus::Unmodified,
        };

        status_map.insert(full_path, status);
    }

    Ok(status_map)
}

pub fn get_git_repo_info(repo_path: &Path) -> Result<GitRepoInfo> {
    let repo = git2::Repository::open(repo_path)?;

    let head = repo.head()?;
    let current_branch = if head.is_branch() {
        head.shorthand().unwrap_or("HEAD").to_string()
    } else {
        "HEAD (detached)".to_string()
    };

    let mut ahead = 0;
    let mut behind = 0;

    if let Ok(local_oid) = head.target().ok_or(anyhow::anyhow!("No target")) {
        if let Ok(branch) = repo.find_branch(&current_branch, git2::BranchType::Local) {
            if let Ok(upstream) = branch.upstream() {
                if let Some(upstream_oid) = upstream.get().target() {
                    if let Ok((a, b)) = repo.graph_ahead_behind(local_oid, upstream_oid) {
                        ahead = a;
                        behind = b;
                    }
                }
            }
        }
    }

    let statuses = repo.statuses(None)?;
    let has_changes = !statuses.is_empty();

    Ok(GitRepoInfo {
        current_branch,
        ahead,
        behind,
        has_changes,
    })
}

pub fn apply_git_status(items: &mut [FileItem], repo_path: &Path) {
    if let Ok(status_map) = get_git_status(repo_path) {
        for item in items.iter_mut() {
            if item.name == ".." {
                continue;
            }

            if let Some(status) = status_map.get(&item.path) {
                item.git_status = Some(status.clone());
            } else {
                item.git_status = Some(GitStatus::Unmodified);
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct MountPoint {
    pub name: String,
    pub mount_point: PathBuf,
    pub device_name: String,
    pub file_system: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
    pub disk_kind: String,
}

impl MountPoint {
    pub fn usage_percentage(&self) -> f32 {
        if self.total_space == 0 {
            return 0.0;
        }
        let used = self.total_space - self.available_space;
        (used as f64 / self.total_space as f64 * 100.0) as f32
    }
}

pub fn get_mount_points() -> Vec<MountPoint> {
    let disks = Disks::new_with_refreshed_list();
    let mut mount_points = Vec::new();

    for disk in disks.list() {
        let mount_point = disk.mount_point().to_path_buf();
        let device_name = disk.name().to_string_lossy().to_string();
        let file_system = disk.file_system().to_string_lossy().to_string();
        let total_space = disk.total_space();
        let available_space = disk.available_space();

        let (is_removable, disk_kind_str) = match disk.kind() {
            DiskKind::HDD => (false, "HDD".to_string()),
            DiskKind::SSD => (false, "SSD".to_string()),
            DiskKind::Unknown(_) => {
                let is_usb = device_name.to_lowercase().contains("usb")
                    || mount_point.to_string_lossy().contains("/media/")
                    || mount_point.to_string_lossy().contains("/run/media/");
                (
                    is_usb,
                    if is_usb {
                        "USB/External".to_string()
                    } else {
                        "Unknown".to_string()
                    },
                )
            }
        };

        let name = if let Some(file_name) = mount_point.file_name() {
            file_name.to_string_lossy().to_string()
        } else {
            mount_point.to_string_lossy().to_string()
        };

        mount_points.push(MountPoint {
            name,
            mount_point,
            device_name,
            file_system,
            total_space,
            available_space,
            is_removable,
            disk_kind: disk_kind_str,
        });
    }

    mount_points.sort_by(|a, b| match (a.is_removable, b.is_removable) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.mount_point.cmp(&b.mount_point),
    });

    mount_points
}
