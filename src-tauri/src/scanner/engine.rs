use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};
use walkdir::WalkDir;

use super::filter::ScanFilter;
use crate::db::{DatabaseManager, VideoAssetRow};

#[derive(Clone)]
pub struct VideoScanEngine {
    db: DatabaseManager,
    is_cancelled: Arc<AtomicBool>,
}

impl VideoScanEngine {
    pub fn new(db: DatabaseManager) -> Self {
        Self {
            db,
            is_cancelled: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn cancel(&self) {
        self.is_cancelled.store(true, Ordering::SeqCst);
    }

    /// 执行多目录递归极速扫描，通过事务分批落盘入库
    pub fn scan_directories<F>(&self, root_dirs: Vec<PathBuf>, mut on_batch: F) -> Result<usize, String>
    where
        F: FnMut(usize, usize, &str),
    {
        self.is_cancelled.store(false, Ordering::SeqCst);

        let mut total_found = 0usize;
        let mut batch_buffer: Vec<VideoAssetRow> = Vec::with_capacity(1000);
        let now_sec = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        for root in root_dirs {
            if !root.exists() {
                continue;
            }

            let root_display = root.to_string_lossy().to_string();
            let walker = WalkDir::new(&root)
                .follow_links(false)
                .into_iter()
                .filter_entry(|entry| {
                    if entry.file_type().is_dir() {
                        // 剪枝判定：一旦命中黑名单，直接剪掉整棵子树
                        !ScanFilter::is_blacklisted_directory(entry.path())
                    } else {
                        true
                    }
                });

            for entry in walker.filter_map(|e| e.ok()) {
                if self.is_cancelled.load(Ordering::Relaxed) {
                    println!("[Scanner] 收到取消信号，中止扫描作业。");
                    return Ok(total_found);
                }

                let path = entry.path();
                if entry.file_type().is_file() && ScanFilter::is_supported_video_format(path) {
                    if let Ok(meta) = entry.metadata() {
                        let file_size = meta.len() as i64;
                        if file_size == 0 {
                            continue;
                        }

                        let file_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("unknown")
                            .to_string();
                        let ext = path
                            .extension()
                            .and_then(|e| e.to_str())
                            .unwrap_or("")
                            .to_lowercase();

                        let created_at = meta
                            .created()
                            .ok()
                            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                            .map(|d| d.as_secs() as i64)
                            .unwrap_or(now_sec);

                        let modified_at = meta
                            .modified()
                            .ok()
                            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
                            .map(|d| d.as_secs() as i64)
                            .unwrap_or(now_sec);

                        batch_buffer.push(VideoAssetRow {
                            id: 0,
                            file_path: path.to_string_lossy().to_string(),
                            file_name,
                            file_size,
                            duration: 0.0,
                            width: 0,
                            height: 0,
                            format: ext,
                            created_at,
                            modified_at,
                            scanned_at: now_sec,
                            status: "discovered".to_string(),
                        });

                        total_found += 1;

                        // 满 500 个文件立即执行一次批量事务提交
                        if batch_buffer.len() >= 500 {
                            let count = batch_buffer.len();
                            let _ = self.db.bulk_upsert_assets(&batch_buffer);
                            on_batch(count, total_found, &root_display);
                            batch_buffer.clear();
                        }
                    }
                }
            }
        }

        // 提交缓冲区剩余数据
        if !batch_buffer.is_empty() {
            let count = batch_buffer.len();
            let _ = self.db.bulk_upsert_assets(&batch_buffer);
            on_batch(count, total_found, "done");
            batch_buffer.clear();
        }

        Ok(total_found)
    }
}
