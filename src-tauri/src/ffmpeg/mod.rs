pub mod extractor;
pub mod pipeline;
pub mod ass;
pub mod hwaccel;

use std::path::{Path, PathBuf};
use std::process::Command;

/// FFmpeg 环境检测与工具支持
#[derive(Debug, Clone)]
pub struct FFmpegContext {
    pub ffmpeg_path: PathBuf,
    pub ffprobe_path: PathBuf,
}

impl FFmpegContext {
    /// 自动发现或指定 FFmpeg 可执行文件
    pub fn discover() -> Result<Self, String> {
        let ffmpeg_bin = Self::find_binary("ffmpeg")
            .ok_or_else(|| "未在系统 PATH 或常见路径中检测到 ffmpeg 二进制程序".to_string())?;
        let ext = if cfg!(windows) { ".exe" } else { "" };
        let ffprobe_bin = Self::find_binary("ffprobe")
            .unwrap_or_else(|| ffmpeg_bin.with_file_name(format!("ffprobe{}", ext)));

        Ok(Self {
            ffmpeg_path: ffmpeg_bin,
            ffprobe_path: ffprobe_bin,
        })
    }

    fn find_binary(name: &str) -> Option<PathBuf> {
        let ext = if cfg!(windows) { ".exe" } else { "" };
        let full_name = format!("{}{}", name, ext);

        // 1. 优先检查显式环境变量
        if let Ok(path) = std::env::var(format!("{}_PATH", name.to_uppercase())) {
            let p = PathBuf::from(&path);
            if p.is_file() {
                return Some(p);
            }
            let p_with_ext = PathBuf::from(format!("{}{}", path, ext));
            if p_with_ext.is_file() {
                return Some(p_with_ext);
            }
        }

        // 2. 进程内遍历 PATH 环境变量（微秒级 stat 检查，0 子进程开销）
        if let Some(path_os) = std::env::var_os("PATH") {
            for dir in std::env::split_paths(&path_os) {
                let candidate = dir.join(&full_name);
                if candidate.is_file() {
                    return Some(candidate);
                }
            }
        }

        // 3. 常见跨平台安装路径 fallback (macOS, Linux, Windows)
        let candidates = [
            format!("/opt/homebrew/bin/{}", name),
            format!("/usr/local/bin/{}", name),
            format!("/usr/bin/{}", name),
            format!("./bin/{}", full_name),
            format!(r"C:\ProgramData\chocolatey\bin\{}", full_name),
            format!(r"C:\ffmpeg\bin\{}", full_name),
            format!(r"C:\Program Files\ffmpeg\bin\{}", full_name),
            format!(r"C:\tools\ffmpeg\bin\{}", full_name),
        ];

        for c in candidates {
            let p = PathBuf::from(c);
            if p.is_file() {
                return Some(p);
            }
        }

        // 4. 最后保底才执行 which/where 子进程
        let search_cmd = if cfg!(windows) { "where" } else { "which" };
        if let Ok(output) = Command::new(search_cmd).arg(&full_name).output() {
            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if let Some(first_line) = stdout.lines().next() {
                    let p = PathBuf::from(first_line.trim());
                    if p.is_file() {
                        return Some(p);
                    }
                }
            }
        }

        None
    }

    /// 获取视频时长（秒）
    pub fn get_media_duration(&self, video_path: &Path) -> Result<f64, String> {
        let output = Command::new(&self.ffprobe_path)
            .args([
                "-v", "error",
                "-show_entries", "format=duration",
                "-of", "default=noprint_wrappers=1:nokey=1",
            ])
            .arg(video_path)
            .output()
            .map_err(|e| format!("执行 ffprobe 失败: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "ffprobe 获取时长失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let dur_str = String::from_utf8_lossy(&output.stdout).trim().to_string();
        dur_str.parse::<f64>().map_err(|e| format!("解析视频时长失败 ({}): {}", dur_str, e))
    }
}
