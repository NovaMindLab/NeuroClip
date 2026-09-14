use std::path::{Path, PathBuf};
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoAssetRow {
    pub id: i64,
    pub file_path: String,
    pub file_name: String,
    pub file_size: i64,
    pub duration: f64,
    pub width: i32,
    pub height: i32,
    pub format: String,
    pub created_at: i64,
    pub modified_at: i64,
    pub scanned_at: i64,
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatCount {
    pub format: String,
    pub count: i64,
    pub percentage: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LibraryStats {
    pub total_count: i64,
    pub total_size_bytes: i64,
    pub total_duration_seconds: f64,
    pub format_counts: Vec<FormatCount>,
}

#[derive(Clone)]
pub struct DatabaseManager {
    db_path: PathBuf,
}

impl DatabaseManager {
    pub fn new(db_path: &Path) -> Result<Self, String> {
        if let Some(parent) = db_path.parent() {
            std::fs::create_dir_all(parent).map_err(|e| format!("创建数据库目录失败: {}", e))?;
        }
        let mgr = Self {
            db_path: db_path.to_path_buf(),
        };
        mgr.init_schema()?;
        let _ = mgr.cleanup_invalid_assets();
        Ok(mgr)
    }

    /// 获取经过最佳 Pragma 调优的 SQLite 独占/线程连接
    pub fn get_connection(&self) -> Result<Connection, String> {
        let conn = Connection::open(&self.db_path)
            .map_err(|e| format!("打开 SQLite 数据库失败 ({:?}): {}", self.db_path, e))?;
        conn.execute_batch(
            r#"
            PRAGMA journal_mode = WAL;
            PRAGMA synchronous = NORMAL;
            PRAGMA cache_size = -64000;
            PRAGMA temp_store = MEMORY;
            PRAGMA busy_timeout = 5000;
            "#,
        )
        .map_err(|e| format!("配置 SQLite PRAGMA 失败: {}", e))?;
        Ok(conn)
    }

    fn init_schema(&self) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS video_assets (
                id            INTEGER PRIMARY KEY AUTOINCREMENT,
                file_path     TEXT NOT NULL,
                file_name     TEXT NOT NULL,
                file_size     INTEGER NOT NULL,
                duration      REAL DEFAULT 0.0,
                width         INTEGER DEFAULT 0,
                height        INTEGER DEFAULT 0,
                format        TEXT NOT NULL,
                created_at    INTEGER NOT NULL,
                modified_at   INTEGER NOT NULL,
                scanned_at    INTEGER NOT NULL,
                status        TEXT NOT NULL DEFAULT 'discovered'
            );

            CREATE UNIQUE INDEX IF NOT EXISTS idx_video_assets_file_path ON video_assets(file_path);
            CREATE INDEX IF NOT EXISTS idx_video_assets_created_at ON video_assets(created_at DESC);
            CREATE INDEX IF NOT EXISTS idx_video_assets_file_name ON video_assets(file_name);
            CREATE INDEX IF NOT EXISTS idx_video_assets_status ON video_assets(status);
            "#,
        )
        .map_err(|e| format!("初始化数据表 DDL 失败: {}", e))?;
        Ok(())
    }

    /// 单一事务内批量 Upsert 插入视频资产列表（万条毫秒级落盘）
    pub fn bulk_upsert_assets(&self, items: &[VideoAssetRow]) -> Result<usize, String> {
        if items.is_empty() {
            return Ok(0);
        }

        let mut conn = self.get_connection()?;
        let tx = conn.transaction().map_err(|e| format!("开启事务失败: {}", e))?;

        let sql = r#"
            INSERT INTO video_assets (
                file_path, file_name, file_size, duration, width, height, format, created_at, modified_at, scanned_at, status
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            ON CONFLICT(file_path) DO UPDATE SET
                file_size = excluded.file_size,
                scanned_at = excluded.scanned_at,
                status = CASE 
                    WHEN video_assets.modified_at != excluded.modified_at THEN 'discovered'
                    ELSE video_assets.status 
                END,
                modified_at = excluded.modified_at
            WHERE video_assets.modified_at != excluded.modified_at OR video_assets.file_size != excluded.file_size;
        "#;

        let mut affected = 0;
        {
            let mut stmt = tx.prepare_cached(sql).map_err(|e| format!("准备 SQL 失败: {}", e))?;
            for item in items {
                let res = stmt.execute(params![
                    item.file_path,
                    item.file_name,
                    item.file_size,
                    item.duration,
                    item.width,
                    item.height,
                    item.format,
                    item.created_at,
                    item.modified_at,
                    item.scanned_at,
                    item.status
                ]).map_err(|e| format!("执行插入语句失败: {}", e))?;
                affected += res;
            }
        }

        tx.commit().map_err(|e| format!("提交事务失败: {}", e))?;
        Ok(affected)
    }

    /// 回写 Probe 提取结果
    pub fn update_probe_metadata(&self, id: i64, duration: f64, width: i32, height: i32) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute(
            "UPDATE video_assets SET duration = ?1, width = ?2, height = ?3, status = 'ready' WHERE id = ?4",
            params![duration, width, height, id],
        )
        .map_err(|e| format!("更新视频元数据失败: {}", e))?;
        Ok(())
    }

    /// 分页条件检索视频列表
    pub fn query_assets(
        &self,
        page: u32,
        page_size: u32,
        search: Option<&str>,
        format_filter: Option<&str>,
        sort_by: Option<&str>,
    ) -> Result<(i64, Vec<VideoAssetRow>), String> {
        let conn = self.get_connection()?;
        let offset = (page.max(1) - 1) * page_size;

        let mut where_clauses = Vec::new();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(s) = search {
            let trimmed = s.trim();
            if !trimmed.is_empty() {
                where_clauses.push("file_name LIKE ?");
                params_vec.push(Box::new(format!("%{}%", trimmed)));
            }
        }

        if let Some(fmt) = format_filter {
            let trimmed = fmt.trim().to_lowercase();
            if !trimmed.is_empty() && trimmed != "all" {
                where_clauses.push("format = ?");
                params_vec.push(Box::new(trimmed));
            }
        }

        let where_sql = if where_clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", where_clauses.join(" AND "))
        };

        // 1. 查询总数
        let count_sql = format!("SELECT COUNT(*) FROM video_assets {}", where_sql);
        let mut count_stmt = conn.prepare(&count_sql).map_err(|e| format!("准备统计查询失败: {}", e))?;
        let param_refs: Vec<&dyn rusqlite::ToSql> = params_vec.iter().map(|b| b.as_ref()).collect();
        let total: i64 = count_stmt
            .query_row(rusqlite::params_from_iter(param_refs.iter().copied()), |row| row.get(0))
            .map_err(|e| format!("查询统计失败: {}", e))?;

        // 2. 排序
        let order_clause = match sort_by {
            Some("size_desc") => "file_size DESC",
            Some("size_asc") => "file_size ASC",
            Some("duration_desc") => "duration DESC",
            Some("name_asc") => "file_name ASC",
            _ => "created_at DESC",
        };

        // 3. 分页查询
        let query_sql = format!(
            "SELECT id, file_path, file_name, file_size, duration, width, height, format, created_at, modified_at, scanned_at, status 
             FROM video_assets {} ORDER BY {} LIMIT ? OFFSET ?",
            where_sql, order_clause
        );

        let mut query_stmt = conn.prepare(&query_sql).map_err(|e| format!("准备分页查询失败: {}", e))?;
        let mut query_params = param_refs;
        let limit_val = page_size as i64;
        let offset_val = offset as i64;
        query_params.push(&limit_val);
        query_params.push(&offset_val);

        let rows = query_stmt
            .query_map(rusqlite::params_from_iter(query_params), |row| {
                Ok(VideoAssetRow {
                    id: row.get(0)?,
                    file_path: row.get(1)?,
                    file_name: row.get(2)?,
                    file_size: row.get(3)?,
                    duration: row.get(4)?,
                    width: row.get(5)?,
                    height: row.get(6)?,
                    format: row.get(7)?,
                    created_at: row.get(8)?,
                    modified_at: row.get(9)?,
                    scanned_at: row.get(10)?,
                    status: row.get(11)?,
                })
            })
            .map_err(|e| format!("执行查询失败: {}", e))?;

        let mut items = Vec::new();
        for r in rows {
            if let Ok(item) = r {
                items.push(item);
            }
        }

        Ok((total, items))
    }

    /// 获取媒体库指标统计
    pub fn get_library_stats(&self) -> Result<LibraryStats, String> {
        let conn = self.get_connection()?;
        
        let (total_count, total_size, total_dur): (i64, i64, f64) = conn
            .query_row(
                "SELECT COUNT(*), COALESCE(SUM(file_size), 0), COALESCE(SUM(duration), 0.0) FROM video_assets",
                [],
                |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
            )
            .unwrap_or((0, 0, 0.0));

        let mut stmt = conn
            .prepare("SELECT format, COUNT(*) as cnt FROM video_assets GROUP BY format ORDER BY cnt DESC")
            .map_err(|e| format!("准备格式统计查询失败: {}", e))?;

        let rows = stmt
            .query_map([], |row| {
                let fmt: String = row.get(0)?;
                let count: i64 = row.get(1)?;
                Ok((fmt, count))
            })
            .map_err(|e| format!("统计格式分布失败: {}", e))?;

        let mut format_counts = Vec::new();
        for r in rows {
            if let Ok((fmt, count)) = r {
                let pct = if total_count > 0 {
                    (count as f32 / total_count as f32) * 100.0
                } else {
                    0.0
                };
                format_counts.push(FormatCount {
                    format: fmt.to_uppercase(),
                    count,
                    percentage: pct,
                });
            }
        }

        Ok(LibraryStats {
            total_count,
            total_size_bytes: total_size,
            total_duration_seconds: total_dur,
            format_counts,
        })
    }

    /// 删除视频记录
    pub fn delete_asset(&self, id: i64) -> Result<(), String> {
        let conn = self.get_connection()?;
        conn.execute("DELETE FROM video_assets WHERE id = ?1", params![id])
            .map_err(|e| format!("删除视频记录失败: {}", e))?;
        Ok(())
    }

    /// 自动清洗并删除历史扫描中误入库的非视频文件（如误将 TypeScript 源码当成 MPEG-TS）
    pub fn cleanup_invalid_assets(&self) -> Result<usize, String> {
        let conn = self.get_connection()?;
        let mut stmt = conn
            .prepare("SELECT id, file_path, format FROM video_assets WHERE LOWER(format) IN ('ts', 'm2ts', 'mts')")
            .map_err(|e| format!("准备清理查询失败: {}", e))?;

        let rows = stmt
            .query_map([], |row| {
                let id: i64 = row.get(0)?;
                let file_path: String = row.get(1)?;
                let format: String = row.get(2)?;
                Ok((id, file_path, format))
            })
            .map_err(|e| format!("查询待清洗记录失败: {}", e))?;

        let mut ids_to_delete = Vec::new();
        for r in rows.flatten() {
            let (id, file_path, _) = r;
            let path = Path::new(&file_path);
            if !path.exists() || !crate::scanner::filter::ScanFilter::is_valid_mpeg_ts(path) {
                ids_to_delete.push(id);
            }
        }

        if !ids_to_delete.is_empty() {
            let tx = conn
                .unchecked_transaction()
                .map_err(|e| format!("开启清理事务失败: {}", e))?;
            for id in &ids_to_delete {
                let _ = tx.execute("DELETE FROM video_assets WHERE id = ?1", params![id]);
            }
            tx.commit().map_err(|e| format!("提交清理事务失败: {}", e))?;
            println!("[Database] 已自动清理 {} 条误入库的 TypeScript 源码记录", ids_to_delete.len());
        }

        Ok(ids_to_delete.len())
    }
}
