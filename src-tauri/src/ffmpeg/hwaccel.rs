use std::process::Command;
use crate::ffmpeg::FFmpegContext;

#[derive(Debug, Clone, PartialEq)]
pub enum HardwareEncoder {
    VideoToolbox, // macOS (Apple Silicon / Intel)
    Nvenc,        // NVIDIA GPU
    Qsv,          // Intel QuickSync
    CpuLibx264,   // 软件编码通用保底
}

impl HardwareEncoder {
    pub fn name(&self) -> &'static str {
        match self {
            HardwareEncoder::VideoToolbox => "h264_videotoolbox",
            HardwareEncoder::Nvenc => "h264_nvenc",
            HardwareEncoder::Qsv => "h264_qsv",
            HardwareEncoder::CpuLibx264 => "libx264",
        }
    }

    /// 针对该编码器的高性能参数
    pub fn default_args(&self) -> Vec<String> {
        match self {
            HardwareEncoder::VideoToolbox => vec![
                "-c:v".to_string(), "h264_videotoolbox".to_string(),
                "-b:v".to_string(), "6500k".to_string(),
                "-maxrate".to_string(), "9000k".to_string(),
                "-bufsize".to_string(), "12000k".to_string(),
                "-pix_fmt".to_string(), "yuv420p".to_string(),
            ],
            HardwareEncoder::Nvenc => vec![
                "-c:v".to_string(), "h264_nvenc".to_string(),
                "-preset".to_string(), "p4".to_string(),
                "-tune".to_string(), "hq".to_string(),
                "-rc".to_string(), "vbr".to_string(),
                "-cq".to_string(), "21".to_string(),
                "-b:v".to_string(), "6500k".to_string(),
                "-pix_fmt".to_string(), "yuv420p".to_string(),
            ],
            HardwareEncoder::Qsv => vec![
                "-c:v".to_string(), "h264_qsv".to_string(),
                "-global_quality".to_string(), "21".to_string(),
                "-b:v".to_string(), "6500k".to_string(),
                "-pix_fmt".to_string(), "nv12".to_string(),
            ],
            HardwareEncoder::CpuLibx264 => vec![
                "-c:v".to_string(), "libx264".to_string(),
                "-preset".to_string(), "fast".to_string(),
                "-crf".to_string(), "20".to_string(),
                "-pix_fmt".to_string(), "yuv420p".to_string(),
            ],
        }
    }

    /// 动态探测当前系统支持的最优硬件加速编码器
    pub fn probe(ctx: &FFmpegContext) -> Self {
        let output = Command::new(&ctx.ffmpeg_path)
            .args(["-v", "error", "-encoders"])
            .output();

        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);

            // 1. macOS 平台优先探测 VideoToolbox
            if cfg!(target_os = "macos") && s.contains("h264_videotoolbox") {
                return HardwareEncoder::VideoToolbox;
            }

            // 2. NVIDIA GPU 探测 NVENC
            if s.contains("h264_nvenc") {
                return HardwareEncoder::Nvenc;
            }

            // 3. Intel 探测 QSV
            if s.contains("h264_qsv") {
                return HardwareEncoder::Qsv;
            }
        }

        HardwareEncoder::CpuLibx264
    }
}
