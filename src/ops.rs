use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum OpKind {
    Copy,
    Move,
    Delete,
    CompressZip,
    CompressTarGz,
    ExtractZip,
    ExtractTar,
}

impl OpKind {
    pub fn label(&self) -> &str {
        match self {
            OpKind::Copy => "Copying",
            OpKind::Move => "Moving",
            OpKind::Delete => "Deleting",
            OpKind::CompressZip | OpKind::CompressTarGz => "Compressing",
            OpKind::ExtractZip | OpKind::ExtractTar => "Extracting",
        }
    }

    pub fn icon(&self) -> &str {
        match self {
            OpKind::Copy => "📋",
            OpKind::Move => "✂️",
            OpKind::Delete => "🗑",
            OpKind::CompressZip => "🗃️",
            OpKind::CompressTarGz => "🗜️",
            OpKind::ExtractZip | OpKind::ExtractTar => "📦",
        }
    }
}

#[derive(Clone, Debug)]
pub enum Operation {
    Copy { sources: Vec<PathBuf>, dest: PathBuf },
    Move { sources: Vec<PathBuf>, dest: PathBuf },
    CompressZip { source: PathBuf, dest_zip: PathBuf },
    CompressTarGz { source: PathBuf, dest_path: PathBuf },
    ExtractZip { archive: PathBuf, dest_dir: PathBuf },
    ExtractTar { archive: PathBuf, dest_dir: PathBuf },
}

#[derive(Clone)]
pub struct OperationState {
    pub kind: OpKind,
    pub title: String,
    pub current_file: String,
    pub bytes_done: u64,
    pub bytes_total: u64,
    pub files_done: u64,
    pub files_total: u64,
    pub finished: bool,
    pub error: Option<String>,
    pub cancelled: bool,
}

struct SharedState {
    state: Mutex<OperationState>,
    cancel: AtomicBool,
    bytes_done: AtomicU64,
    files_done: AtomicU64,
}

impl SharedState {
    fn update_bytes(&self, delta: u64) {
        self.bytes_done.fetch_add(delta, Ordering::Relaxed);
        if let Ok(mut s) = self.state.lock() {
            s.bytes_done = self.bytes_done.load(Ordering::Relaxed);
        }
    }

    fn increment_files(&self) {
        self.files_done.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut s) = self.state.lock() {
            s.files_done = self.files_done.load(Ordering::Relaxed);
        }
    }

    fn set_current_file(&self, name: &str) {
        if let Ok(mut s) = self.state.lock() {
            s.current_file = name.to_string();
        }
    }

    fn is_cancelled(&self) -> bool {
        self.cancel.load(Ordering::Relaxed)
    }
}

pub struct OpRunner {
    shared: Arc<SharedState>,
    handle: Option<JoinHandle<()>>,
}

impl OpRunner {
    pub fn start(operation: Operation) -> Result<Self> {
        let (kind, title, bytes_total, files_total) = scan_operation(&operation)?;

        let shared = Arc::new(SharedState {
            state: Mutex::new(OperationState {
                kind,
                title,
                current_file: String::new(),
                bytes_done: 0,
                bytes_total,
                files_done: 0,
                files_total,
                finished: false,
                error: None,
                cancelled: false,
            }),
            cancel: AtomicBool::new(false),
            bytes_done: AtomicU64::new(0),
            files_done: AtomicU64::new(0),
        });

        let shared_thread = Arc::clone(&shared);
        let handle = std::thread::spawn(move || {
            let result = run_operation(&shared_thread, operation);
            if let Ok(mut s) = shared_thread.state.lock() {
                s.finished = true;
                match result {
                    Ok(()) => {}
                    Err(e) => {
                        if shared_thread.is_cancelled() {
                            s.cancelled = true;
                        } else {
                            s.error = Some(format!("{}", e));
                        }
                    }
                }
            }
        });

        Ok(OpRunner {
            shared,
            handle: Some(handle),
        })
    }

    pub fn snapshot(&self) -> OperationState {
        self.shared.state.lock().map(|s| s.clone()).unwrap()
    }

    pub fn is_finished(&self) -> bool {
        self.shared
            .state
            .lock()
            .map(|s| s.finished)
            .unwrap_or(true)
    }

    pub fn cancel(&self) {
        self.shared.cancel.store(true, Ordering::Relaxed);
    }

    pub fn join(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

fn count_dir(path: &Path, bytes: &mut u64, files: &mut u64) {
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let Ok(ft) = entry.file_type() else { continue };
            if ft.is_dir() {
                count_dir(&entry.path(), bytes, files);
            } else if let Ok(meta) = entry.metadata() {
                *bytes += meta.len();
                *files += 1;
            }
        }
    }
}

fn scan_operation(op: &Operation) -> Result<(OpKind, String, u64, u64)> {
    match op {
        Operation::Copy { sources, .. } | Operation::Move { sources, .. } => {
            let kind = match op {
                Operation::Copy { .. } => OpKind::Copy,
                _ => OpKind::Move,
            };
            let mut bytes_total = 0u64;
            let mut files_total = 0u64;
            for src in sources {
                if src.is_dir() {
                    count_dir(src, &mut bytes_total, &mut files_total);
                } else if let Ok(meta) = src.metadata() {
                    bytes_total += meta.len();
                    files_total += 1;
                }
            }
            let title = sources
                .first()
                .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| "files".to_string());
            Ok((kind, title, bytes_total, files_total))
        }
        Operation::CompressZip { source, dest_zip: dest }
        | Operation::CompressTarGz { source, dest_path: dest } => {
            let kind = match op {
                Operation::CompressZip { .. } => OpKind::CompressZip,
                _ => OpKind::CompressTarGz,
            };
            let archive_fallback = match op {
                Operation::CompressZip { .. } => "archive.zip",
                _ => "archive.tar.gz",
            };
            let mut bytes_total = 0u64;
            let mut files_total = 0u64;
            if source.is_dir() {
                count_dir(source, &mut bytes_total, &mut files_total);
            } else if let Ok(meta) = source.metadata() {
                bytes_total += meta.len();
                files_total += 1;
            }
            let title = dest
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| archive_fallback.to_string());
            Ok((kind, title, bytes_total, files_total))
        }
        Operation::ExtractZip { archive, .. } | Operation::ExtractTar { archive, .. } => {
            let kind = match op {
                Operation::ExtractZip { .. } => OpKind::ExtractZip,
                _ => OpKind::ExtractTar,
            };
            let mut bytes_total = 0u64;
            let mut files_total = 0u64;
            if kind == OpKind::ExtractZip {
                if let Ok(file) = fs::File::open(archive) {
                    if let Ok(mut z) = zip::ZipArchive::new(file) {
                        for i in 0..z.len() {
                            if let Ok(f) = z.by_index(i) {
                                bytes_total += f.size();
                                files_total += 1;
                            }
                        }
                    }
                }
            } else if let Ok(meta) = archive.metadata() {
                // compressed size as approximation
                bytes_total = meta.len() * 3;
                files_total = 10;
            }
            let title = archive
                .file_name()
                .map(|n| n.to_string_lossy().to_string())
                .unwrap_or_else(|| "archive".to_string());
            Ok((kind, title, bytes_total, files_total))
        }
    }
}

fn report_copied(shared: &SharedState, from: &Path, to: &Path) -> Result<()> {
    let len = fs::metadata(from).map(|m| m.len()).unwrap_or(0);
    fs::copy(from, to)?;
    shared.update_bytes(len);
    shared.increment_files();
    Ok(())
}

fn copy_dir_bg(shared: &SharedState, src: &Path, dst: &Path) -> Result<()> {
    if shared.is_cancelled() {
        anyhow::bail!("cancelled");
    }
    fs::create_dir_all(dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ft = entry.file_type()?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if shared.is_cancelled() {
            anyhow::bail!("cancelled");
        }
        if ft.is_dir() {
            copy_dir_bg(shared, &src_path, &dst_path)?;
        } else {
            if let Some(name) = src_path.file_name() {
                shared.set_current_file(&name.to_string_lossy());
            }
            report_copied(shared, &src_path, &dst_path)?;
        }
    }
    Ok(())
}

fn run_operation(shared: &SharedState, op: Operation) -> Result<()> {
    match op {
        Operation::Copy { sources, dest } => {
            fs::create_dir_all(&dest)?;
            for src in sources {
                if shared.is_cancelled() {
                    anyhow::bail!("cancelled");
                }
                let name = src
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("Invalid source path"))?;
                let dest_path = dest.join(name);
                shared.set_current_file(&name.to_string_lossy());
                if src.is_dir() {
                    copy_dir_bg(shared, &src, &dest_path)?;
                } else {
                    report_copied(shared, &src, &dest_path)?;
                }
            }
            Ok(())
        }
        Operation::Move { sources, dest } => {
            fs::create_dir_all(&dest)?;
            for src in sources {
                if shared.is_cancelled() {
                    anyhow::bail!("cancelled");
                }
                let name = src
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("Invalid source path"))?;
                let dest_path = dest.join(name);
                shared.set_current_file(&name.to_string_lossy());
                if let Ok(meta) = src.metadata() {
                    shared.update_bytes(meta.len());
                }
                fs::rename(&src, &dest_path)?;
                shared.increment_files();
            }
            Ok(())
        }
        Operation::CompressZip { source, dest_zip } => {
            use std::io::Write;
            let file = fs::File::create(&dest_zip)?;
            let mut zip = zip::ZipWriter::new(file);
            let options = zip::write::FileOptions::default()
                .compression_method(zip::CompressionMethod::Deflated)
                .unix_permissions(0o755);

            if source.is_file() {
                let name = source
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
                    .to_string_lossy();
                shared.set_current_file(&name);
                zip.start_file(name.as_ref(), options)?;
                let len = fs::metadata(&source).map(|m| m.len()).unwrap_or(0);
                let content = fs::read(&source)?;
                zip.write_all(&content)?;
                shared.update_bytes(len);
                shared.increment_files();
            } else if source.is_dir() {
                compress_dir_bg(shared, &mut zip, &source, &source, options)?;
            }

            zip.finish()?;
            Ok(())
        }
        Operation::CompressTarGz { source, dest_path } => {
            let file = fs::File::create(&dest_path)?;
            let enc = flate2::write::GzEncoder::new(file, flate2::Compression::default());
            let mut tar = tar::Builder::new(enc);
            tar.follow_symlinks(false);

            if source.is_file() {
                let name = source
                    .file_name()
                    .ok_or_else(|| anyhow::anyhow!("Invalid filename"))?
                    .to_string_lossy()
                    .to_string();
                shared.set_current_file(&name);
                tar.append_path_with_name(&source, name.as_str())?;
                let len = fs::metadata(&source).map(|m| m.len()).unwrap_or(0);
                shared.update_bytes(len);
                shared.increment_files();
            } else if source.is_dir() {
                tar_dir_bg(shared, &mut tar, &source, &source)?;
            } else {
                anyhow::bail!("Source is neither a file nor a directory");
            }

            let enc = tar.into_inner()?;
            enc.finish()?;
            Ok(())
        }
        Operation::ExtractZip { archive, dest_dir } => {
            fs::create_dir_all(&dest_dir)?;
            let file = fs::File::open(&archive)?;
            let mut z = zip::ZipArchive::new(file)?;
            for i in 0..z.len() {
                if shared.is_cancelled() {
                    anyhow::bail!("cancelled");
                }
                let mut f = z.by_index(i)?;
                shared.set_current_file(f.name());
                let outpath = dest_dir.join(f.name());
                if f.name().ends_with('/') {
                    fs::create_dir_all(&outpath)?;
                } else {
                    if let Some(p) = outpath.parent() {
                        if !p.exists() {
                            fs::create_dir_all(p)?;
                        }
                    }
                    let mut outfile = fs::File::create(&outpath)?;
                    std::io::copy(&mut f, &mut outfile)?;
                    #[cfg(unix)]
                    {
                        use std::os::unix::fs::PermissionsExt;
                        if let Some(mode) = f.unix_mode() {
                            let _ = fs::set_permissions(
                                &outpath,
                                fs::Permissions::from_mode(mode),
                            );
                        }
                    }
                    shared.update_bytes(f.size());
                    shared.increment_files();
                }
            }
            Ok(())
        }
        Operation::ExtractTar { archive, dest_dir } => {
            fs::create_dir_all(&dest_dir)?;
            shared.set_current_file("unpacking archive...");
            crate::filesystem::extract_tar_archive(&archive, &dest_dir)?;
            shared.increment_files();
            Ok(())
        }
    }
}

fn compress_dir_bg(
    shared: &SharedState,
    zip: &mut zip::ZipWriter<std::fs::File>,
    base_path: &Path,
    current_path: &Path,
    options: zip::write::FileOptions,
) -> Result<()> {
    use std::io::Write;
    if shared.is_cancelled() {
        anyhow::bail!("cancelled");
    }
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(base_path)?;
        if shared.is_cancelled() {
            anyhow::bail!("cancelled");
        }
        if path.is_file() {
            shared.set_current_file(&name.to_string_lossy());
            zip.start_file(name.to_string_lossy().as_ref(), options)?;
            let len = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            let content = fs::read(&path)?;
            zip.write_all(&content)?;
            shared.update_bytes(len);
            shared.increment_files();
        } else if path.is_dir() {
            zip.add_directory(name.to_string_lossy().as_ref(), options)?;
            compress_dir_bg(shared, zip, base_path, &path, options)?;
        }
    }
    Ok(())
}

fn tar_dir_bg(
    shared: &SharedState,
    tar: &mut tar::Builder<flate2::write::GzEncoder<std::fs::File>>,
    base_path: &Path,
    current_path: &Path,
) -> Result<()> {
    if shared.is_cancelled() {
        anyhow::bail!("cancelled");
    }
    for entry in fs::read_dir(current_path)? {
        let entry = entry?;
        let path = entry.path();
        let name = path.strip_prefix(base_path)?;
        if shared.is_cancelled() {
            anyhow::bail!("cancelled");
        }
        if path.is_file() {
            shared.set_current_file(&name.to_string_lossy());
            tar.append_path_with_name(&path, name)?;
            let len = fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
            shared.update_bytes(len);
            shared.increment_files();
        } else if path.is_dir() {
            tar.append_dir(name, &path)?;
            tar_dir_bg(shared, tar, base_path, &path)?;
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compress_tar_gz_roundtrip() {
        let tmp = std::env::temp_dir().join("filane_ops_test");
        let _ = fs::remove_dir_all(&tmp);
        let src = tmp.join("src_dir");
        fs::create_dir_all(src.join("sub")).unwrap();
        fs::write(src.join("a.txt"), b"hello").unwrap();
        fs::write(src.join("sub").join("b.txt"), b"world").unwrap();

        let dest = tmp.join("out.tar.gz");
        let shared = SharedState {
            state: Mutex::new(OperationState {
                kind: OpKind::CompressTarGz,
                title: "out.tar.gz".into(),
                current_file: String::new(),
                bytes_done: 0,
                bytes_total: 100,
                files_done: 0,
                files_total: 2,
                finished: false,
                error: None,
                cancelled: false,
            }),
            cancel: AtomicBool::new(false),
            bytes_done: AtomicU64::new(0),
            files_done: AtomicU64::new(0),
        };
        run_operation(
            &shared,
            Operation::CompressTarGz {
                source: src.clone(),
                dest_path: dest.clone(),
            },
        )
        .unwrap();

        assert!(dest.exists());
        let f = fs::File::open(&dest).unwrap();
        let dec = flate2::read::GzDecoder::new(f);
        let mut ar = tar::Archive::new(dec);
        let names: Vec<String> = ar
            .entries()
            .unwrap()
            .map(|e| e.unwrap().path().unwrap().to_string_lossy().to_string())
            .collect();
        println!("entries: {:?}", names);
        assert!(!names.is_empty());
        assert!(names.iter().any(|n| n.contains("a.txt")));
        assert!(names.iter().any(|n| n.contains("b.txt")));
        assert_eq!(shared.files_done.load(Ordering::Relaxed), 2);
        assert!(shared.bytes_done.load(Ordering::Relaxed) >= 10);

        let _ = fs::remove_dir_all(&tmp);
    }
}
