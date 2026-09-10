use std::path::{Path, PathBuf};
use std::process::Command;
use serde::{Deserialize, Serialize};
use crate::ffmpeg::FFmpegContext;
use crate::ffmpeg::hwaccel::HardwareEncoder;

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
    pub ass_subtitles_path: Option<PathBuf>,
    pub bgm_path: Option<PathBuf>,
    pub is_vertical_9_16: bool,
    pub encoder: Option<HardwareEncoder>,
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
            ass_subtitles_path: None,
            bgm_path: None,
            is_vertical_9_16: false,
            encoder: None,
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

    /// 构造工业级混剪混流 FFmpeg 命令参数
    /// 包含：
    /// 1. 毫秒级时间裁剪
    /// 2. 9:16 动态模糊背景重构 (boxblur + 智能居中)
    /// 3. ASS / SRT 动效硬字幕烧录
    /// 4. 2 轨 / 3 轨 sidechaincompress 侧链闪避与 amix 混音 (原声下压 12dB, BGM 下压 15dB)
    /// 5. 硬件加速或软件编码高清压制
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
            "-ss".to_string(),
            start_str,
            "-t".to_string(),
            duration_str,
            "-i".to_string(),
            video_path.to_string_lossy().to_string(),
            "-i".to_string(),
            voiceover_path.to_string_lossy().to_string(),
        ];

        // 若挂载第 3 轨 BGM 伴奏
        let has_bgm = options.bgm_path.is_some();
        if let Some(ref bgm) = options.bgm_path {
            args.push("-i".to_string());
            args.push(bgm.to_string_lossy().to_string());
        }

        // 1. 构造音频滤镜图
        let audio_filter = if has_bgm {
            // 3 轨混音：解说压低原声 12dB，解说压低 BGM 15dB
            format!(
                "[0:a][1:a]sidechaincompress=threshold={}:ratio={}:attack={}:release={}[bg_ducked];\
                 [2:a][1:a]sidechaincompress=threshold=0.10:ratio=5:attack=15:release=300,volume=0.45[bgm_ducked];\
                 [bg_ducked][bgm_ducked][1:a]amix=inputs=3:duration=first:dropout_transition=2[aout]",
                options.ducking_threshold, options.ducking_ratio, options.attack_ms, options.release_ms
            )
        } else {
            // 2 轨混音
            format!(
                "[0:a][1:a]sidechaincompress=threshold={}:ratio={}:attack={}:release={}[bg_ducked];\
                 [bg_ducked][1:a]amix=inputs=2:duration=first:dropout_transition=2[aout]",
                options.ducking_threshold, options.ducking_ratio, options.attack_ms, options.release_ms
            )
        };

        // 2. 构造视频滤镜图 (9:16 动态模糊背景 + 动效字幕)
        let mut filter_complex = String::new();
        let mut current_v_label = "0:v".to_string();

        if options.is_vertical_9_16 {
            filter_complex.push_str(
                "[0:v]split=2[bg][fg];\
                 [bg]scale=1080:1920:force_original_aspect_ratio=increase,crop=1080:1920,boxblur=25:5,eq=brightness=-0.15[bgblur];\
                 [fg]scale=1080:1920:force_original_aspect_ratio=decrease[fgscale];\
                 [bgblur][fgscale]overlay=(W-w)/2:(H-h)/2[vbase];"
            );
            current_v_label = "vbase".to_string();
        }

        // 字幕处理：优先使用 ASS 动效字幕，其次使用 SRT
        let mut has_subtitles = false;
        if let Some(ref ass) = options.ass_subtitles_path {
            let ass_escaped = ass.to_string_lossy().replace('\\', "/").replace(':', "\\:");
            filter_complex.push_str(&format!(
                "[{}]ass='{}'[vout];",
                current_v_label, ass_escaped
            ));
            has_subtitles = true;
        } else if let Some(ref srt) = options.subtitles_path {
            let srt_escaped = srt.to_string_lossy().replace('\\', "/").replace(':', "\\:");
            filter_complex.push_str(&format!(
                "[{}]subtitles='{}':force_style='FontSize=20,PrimaryColour=&H00FFFFFF,OutlineColour=&H00000000,BorderStyle=3,MarginV=30'[vout];",
                current_v_label, srt_escaped
            ));
            has_subtitles = true;
        }

        filter_complex.push_str(&audio_filter);

        args.push("-filter_complex".to_string());
        args.push(filter_complex);

        if has_subtitles {
            args.push("-map".to_string());
            args.push("[vout]".to_string());
        } else if options.is_vertical_9_16 {
            args.push("-map".to_string());
            args.push("[vbase]".to_string());
        } else {
            args.push("-map".to_string());
            args.push("0:v".to_string());
        }

        args.push("-map".to_string());
        args.push("[aout]".to_string());

        // 3. 编码参数 (硬件加速或 CPU 保底)
        let enc = options.encoder.clone().unwrap_or(HardwareEncoder::CpuLibx264);
        args.extend(enc.default_args());

        // 音频编码
        args.extend_from_slice(&[
            "-c:a".to_string(), "aac".to_string(),
            "-b:a".to_string(), "192k".to_string(),
            "-movflags".to_string(), "+faststart".to_string(),
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
