use std::path::Path;
use std::process::Command;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct FFprobeOutput {
    streams: Option<Vec<StreamEntry>>,
    format: Option<FormatEntry>,
}

#[derive(Debug, Deserialize)]
struct StreamEntry {
    width: Option<i32>,
    height: Option<i32>,
    duration: Option<String>,
}

#[derive(Debug, Deserialize)]
struct FormatEntry {
    duration: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VideoProbeResult {
    pub duration: f64,
    pub width: i32,
    pub height: i32,
}

pub struct LightProbeEngine;

impl LightProbeEngine {
    /// 调用 ffprobe 轻量提取，带参数与异常保护
    pub fn probe_video(ffprobe_path: &Path, video_path: &Path) -> Result<VideoProbeResult, String> {
        let output = Command::new(ffprobe_path)
            .args([
                "-v", "error",
                "-select_streams", "v:0",
                "-show_entries", "stream=width,height,duration:format=duration",
                "-of", "json",
            ])
            .arg(video_path)
            .output()
            .map_err(|e| format!("执行 ffprobe 进程失败: {}", e))?;

        if !output.status.success() {
            return Err(format!(
                "ffprobe 获取信息失败: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }

        let parsed: FFprobeOutput = serde_json::from_slice(&output.stdout)
            .map_err(|e| format!("解析 ffprobe JSON 结构失败: {}", e))?;

        let first_stream = parsed.streams.as_ref().and_then(|s| s.first());
        let width = first_stream.and_then(|s| s.width).unwrap_or(1920);
        let height = first_stream.and_then(|s| s.height).unwrap_or(1080);

        let duration_str = first_stream
            .and_then(|s| s.duration.clone())
            .or_else(|| parsed.format.and_then(|f| f.duration))
            .unwrap_or_else(|| "0.0".to_string());

        let duration = duration_str.parse::<f64>().unwrap_or(0.0);

        Ok(VideoProbeResult {
            duration,
            width,
            height,
        })
    }
}
