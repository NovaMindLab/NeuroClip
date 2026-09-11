use std::process::Command;
use crate::ffmpeg::FFmpegContext;

#[derive(Debug, Clone, PartialEq)]
pub enum HardwareEncoder {
    VideoToolbox, // macOS (Apple Silicon / Intel)
    Nvenc,        // NVIDIA GPU (Windows / Linux)
    Qsv,          // Intel QuickSync (Windows / Linux)
    Amf,          // AMD Radeon GPU (Windows / Linux)
    CpuLibx264,   // 软件编码通用保底
}

impl HardwareEncoder {
    pub fn name(&self) -> &'static str {
        match self {
            HardwareEncoder::VideoToolbox => "h264_videotoolbox",
            HardwareEncoder::Nvenc => "h264_nvenc",
            HardwareEncoder::Qsv => "h264_qsv",
            HardwareEncoder::Amf => "h264_amf",
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
            HardwareEncoder::Amf => vec![
                "-c:v".to_string(), "h264_amf".to_string(),
                "-quality".to_string(), "speed".to_string(),
                "-rc".to_string(), "vbr_latency".to_string(),
                "-b:v".to_string(), "6500k".to_string(),
                "-pix_fmt".to_string(), "yuv420p".to_string(),
            ],
            HardwareEncoder::CpuLibx264 => vec![
                "-c:v".to_string(), "libx264".to_string(),
                "-preset".to_string(), "fast".to_string(),
                "-crf".to_string(), "20".to_string(),
                "-pix_fmt".to_string(), "yuv420p".to_string(),
            ],
        }
    }

    /// 动态探测当前系统支持的最优硬件加速编码器（带平台 0ms Fast-Path）
    pub fn probe(ctx: &FFmpegContext) -> Self {
        // 1. macOS 平台 Fast-Path：只要检测到 FFmpeg 存在，原生 100% 支持 VideoToolbox 硬编，0ms 瞬返
        if cfg!(target_os = "macos") && ctx.ffmpeg_path.is_file() {
            return HardwareEncoder::VideoToolbox;
        }

        // 2. Windows 平台极速驱动检查（通过 System32 驱动文件直查，0.1ms 代替 350ms 的子进程）
        if cfg!(target_os = "windows") {
            if std::path::Path::new(r"C:\Windows\System32\nvencodeapi64.dll").is_file() {
                return HardwareEncoder::Nvenc;
            }
            if std::path::Path::new(r"C:\Windows\System32\amfrt64.dll").is_file() {
                return HardwareEncoder::Amf;
            }
        }

        // 3. 通用外部子进程深度探测（保底）
        let output = Command::new(&ctx.ffmpeg_path)
            .args(["-v", "error", "-encoders"])
            .output();

        if let Ok(out) = output {
            let s = String::from_utf8_lossy(&out.stdout);

            if cfg!(target_os = "macos") && s.contains("h264_videotoolbox") {
                return HardwareEncoder::VideoToolbox;
            }
            if s.contains("h264_nvenc") {
                return HardwareEncoder::Nvenc;
            }
            if s.contains("h264_amf") {
                return HardwareEncoder::Amf;
            }
            if s.contains("h264_qsv") {
                return HardwareEncoder::Qsv;
            }
        }

        HardwareEncoder::CpuLibx264
    }
}
