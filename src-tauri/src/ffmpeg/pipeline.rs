use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use crate::ffmpeg::FFmpegContext;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubtitleCue {
    pub start_sec: f64,
    pub end_sec: f64,
    pub text: String,
}

#[derive(Debug, Clone)]
pub struct RemuxOptions {
    pub start_time: f64,
    pub end_time: f64,
    pub ducking_ratio: f32,       // e.g. 4.0 ~ 5.0 (压低 12dB)
    pub ducking_threshold: f32,   // e.g. 0.125
    pub attack_ms: u32,           // e.g. 20ms
    pub release_ms: u32,          // e.g. 250ms
    pub subtitles_path: Option<PathBuf>,
}

impl Default for RemuxOptions {
    fn default() -> Self {
        Self {
            start_time: 0.0,
            end_time: 30.0,
            ducking_ratio: 4.0,
            ducking_threshold: 0.125,
            attack_ms: 20,
            release_ms: 250,
            subtitles_path: None,
        }
    }
}

pub struct RemuxPipeline<'a> {
    ctx: &'a FFmpegContext,
}

impl<'a> RemuxPipeline<'a> {
    pub fn new(ctx: &'a FFmpegContext) -> Self {
        Self { ctx }
    }

    /// 将字幕条目写入临时 SRT 文件
    pub fn generate_srt_file(cues: &[SubtitleCue], output_srt: &Path) -> Result<(), String> {
        let mut content = String::new();
        for (i, cue) in cues.iter().enumerate() {
            let start = Self::format_srt_time(cue.start_sec);
            let end = Self::format_srt_time(cue.end_sec);
            content.push_str(&format!("{}\n{} --> {}\n{}\n\n", i + 1, start, end, cue.text));
        }
        std::fs::write(output_srt, content).map_err(|e| format!("写入 SRT 失败: {}", e))
    }

    fn format_srt_time(seconds: f64) -> String {
        let total_ms = (seconds * 1000.0).round() as u64;
        let ms = total_ms % 1000;
        let total_sec = total_ms / 1000;
        let s = total_sec % 60;
        let total_min = total_sec / 60;
        let m = total_min % 60;
        let h = total_min / 60;
        format!("{:02}:{:02}:{:02},{:03}", h, m, s, ms)
    }

    /// 构造混剪混流 FFmpeg 命令参数
    /// 包含：
    /// 1. 毫秒级时间裁剪
    /// 2. sidechaincompress 侧链音频闪避（解说出声时背景音平滑下压 12dB）
    /// 3. amix 解说与背景混音
    /// 4. subtitles 硬字幕滤镜烧录
    /// 5. libx264 + aac 压制
    pub fn build_remux_args(
        video_path: &Path,
        voiceover_path: &Path,
        output_path: &Path,
        options: &RemuxOptions,
    ) -> Vec<String> {
        let start_str = format!("{:.3}", options.start_time);
        let duration_str = format!("{:.3}", (options.end_time - options.start_time).max(0.1));

        let mut args = vec![
            "-y".to_string(),
            // 视频精准起止时间裁剪
            "-ss".to_string(),
            start_str,
            "-t".to_string(),
            duration_str,
            "-i".to_string(),
            video_path.to_string_lossy().to_string(),
            // 输入旁白音频
            "-i".to_string(),
            voiceover_path.to_string_lossy().to_string(),
        ];

        // 构造音频 filter: sidechaincompress + amix
        let audio_filter = format!(
            "[0:a][1:a]sidechaincompress=threshold={}:ratio={}:attack={}:release={}[bg_ducked];[bg_ducked][1:a]amix=inputs=2:duration=first:dropout_transition=2[aout]",
            options.ducking_threshold, options.ducking_ratio, options.attack_ms, options.release_ms
        );

        // 构造视频 filter: subtitles 硬字幕 (若提供)
        let (filter_complex, has_video_filter) = if let Some(ref srt) = options.subtitles_path {
            // 转义路径中的冒号与反斜杠，供 ffmpeg filter 使用
            let srt_escaped = srt.to_string_lossy().replace('\\', "/").replace(':', "\\:");
            let vf = format!(
                "[0:v]subtitles='{}':force_style='FontSize=20,PrimaryColour=&H00FFFFFF,OutlineColour=&H00000000,BorderStyle=3,MarginV=30'[vout];",
                srt_escaped
            );
            (format!("{}{}", vf, audio_filter), true)
        } else {
            (audio_filter, false)
        };

        args.push("-filter_complex".to_string());
        args.push(filter_complex);

        if has_video_filter {
            args.push("-map".to_string());
            args.push("[vout]".to_string());
        } else {
            args.push("-map".to_string());
            args.push("0:v".to_string());
        }

        args.push("-map".to_string());
        args.push("[aout]".to_string());

        // 编码参数
        args.extend_from_slice(&[
            "-c:v".to_string(),
            "libx264".to_string(),
            "-preset".to_string(),
            "fast".to_string(),
            "-crf".to_string(),
            "20".to_string(),
            "-c:a".to_string(),
            "aac".to_string(),
            "-b:a".to_string(),
            "192k".to_string(),
            "-pix_fmt".to_string(),
            "yuv420p".to_string(),
            "-movflags".to_string(),
            "+faststart".to_string(),
            output_path.to_string_lossy().to_string(),
        ]);

        args
    }

    /// 执行混流与压制任务
    pub fn execute_remux(
        &self,
        video_path: &Path,
        voiceover_path: &Path,
        output_path: &Path,
        options: &RemuxOptions,
    ) -> Result<PathBuf, String> {
        let args = Self::build_remux_args(video_path, voiceover_path, output_path, options);

        let output = Command::new(&self.ctx.ffmpeg_path)
            .args(&args)
            .output()
            .map_err(|e| format!("启动 FFmpeg 混剪压制失败: {}", e))?;

        if !output.status.success() {
            let err_log = String::from_utf8_lossy(&output.stderr);
            return Err(format!("FFmpeg 混剪压制错误: {}", err_log));
        }

        Ok(output_path.to_path_buf())
    }
}
