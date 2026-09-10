use std::path::{Path, PathBuf};
use std::process::Command;
use crate::ffmpeg::FFmpegContext;

/// 音视频流抽离器
pub struct MediaExtractor<'a> {
    ctx: &'a FFmpegContext,
}

impl<'a> MediaExtractor<'a> {
    pub fn new(ctx: &'a FFmpegContext) -> Self {
        Self { ctx }
    }

    /// 提取单声道 16kHz WAV 格式音频
    /// 参数：
    /// - `video_path`: 输入原始长视频路径
    /// - `output_wav_path`: 导出的 16kHz 16-bit PCM WAV 路径
    pub fn extract_16k_mono_wav(
        &self,
        video_path: &Path,
        output_wav_path: &Path,
    ) -> Result<PathBuf, String> {
        if !video_path.exists() {
            return Err(format!("输入视频文件不存在: {:?}", video_path));
        }

        // 确保输出目录存在
        if let Some(parent) = output_wav_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        // ffmpeg -y -i input.mp4 -vn -ac 1 -ar 16000 -c:a pcm_s16le output.wav
        let status = Command::new(&self.ctx.ffmpeg_path)
            .arg("-y")
            .arg("-i")
            .arg(video_path)
            .arg("-vn")
            .arg("-ac")
            .arg("1")
            .arg("-ar")
            .arg("16000")
            .arg("-c:a")
            .arg("pcm_s16le")
            .arg(output_wav_path)
            .status()
            .map_err(|e| format!("启动 FFmpeg 抽取音频失败: {}", e))?;

        if !status.success() {
            return Err(format!("FFmpeg 抽离音频非零退出码: {:?}", status.code()));
        }

        Ok(output_wav_path.to_path_buf())
    }

    /// 降采样抽取低分辨率低帧率图像流 (用于视觉转场检测 TransNetV2)
    /// 例如：以 4fps 抽取 128x128 像素的 RGB 关键图像帧
    pub fn extract_lowres_frames(
        &self,
        video_path: &Path,
        output_pattern: &Path,
        fps: f32,
    ) -> Result<(), String> {
        if !video_path.exists() {
            return Err(format!("输入视频不存在: {:?}", video_path));
        }

        let status = Command::new(&self.ctx.ffmpeg_path)
            .arg("-y")
            .arg("-i")
            .arg(video_path)
            .arg("-vf")
            .arg(format!("fps={},scale=128:128", fps))
            .arg("-q:v")
            .arg("3")
            .arg(output_pattern)
            .status()
            .map_err(|e| format!("抽取视觉降采样帧失败: {}", e))?;

        if !status.success() {
            return Err("抽取视觉流帧失败".to_string());
        }

        Ok(())
    }
}
