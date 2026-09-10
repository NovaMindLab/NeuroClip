use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BgmType {
    HypeTrap,      // 燃系电竞 Trap 鼓点
    SuspenseDrone, // 悬念紧迫心跳
    FunnyMeme,     // 幽默搞怪反转
    None,          // 无伴奏
}

impl Default for BgmType {
    fn default() -> Self {
        BgmType::HypeTrap
    }
}

impl BgmType {
    pub fn name_zh(&self) -> &'static str {
        match self {
            BgmType::HypeTrap => "燃系电竞 Trap",
            BgmType::SuspenseDrone => "悬念紧迫低音",
            BgmType::FunnyMeme => "幽默反转搞怪",
            BgmType::None => "无背景音乐",
        }
    }

    /// 内置生成符合情绪风格的免版权背景伴奏 WAV 文件 (用于离线与免外置下载)
    pub fn generate_bgm_track(&self, output_path: &Path, duration_sec: f64) -> Result<PathBuf, String> {
        if let Some(parent) = output_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }

        let sample_rate = 16000u32;
        let total_samples = (duration_sec * sample_rate as f64) as usize;

        let spec = hound::WavSpec {
            channels: 1,
            sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = hound::WavWriter::create(output_path, spec)
            .map_err(|e| format!("创建 BGM WAV 失败: {}", e))?;

        let bpm = match self {
            BgmType::HypeTrap => 140.0,
            BgmType::SuspenseDrone => 90.0,
            BgmType::FunnyMeme => 120.0,
            BgmType::None => 100.0,
        };

        let beat_samples = ((60.0 / bpm) * sample_rate as f64) as usize;

        for i in 0..total_samples {
            let t = (i as f64) / (sample_rate as f64);
            let beat_pos = i % beat_samples;
            let beat_decay = (-(beat_pos as f64) / (beat_samples as f64 * 0.4)).exp();

            let sample_val = match self {
                BgmType::HypeTrap => {
                    // 808 重低音 (55Hz) + 弱踩镲
                    let kick = (2.0 * std::f64::consts::PI * 55.0 * t).sin() * beat_decay * 0.6;
                    let hihat = if beat_pos < 300 { ((i % 13) as f64 / 13.0 - 0.5) * 0.15 } else { 0.0 };
                    kick + hihat
                }
                BgmType::SuspenseDrone => {
                    // 低频 Drone (45Hz + 60Hz 拍频心跳)
                    let drone = (2.0 * std::f64::consts::PI * 45.0 * t).sin() * 0.35
                        + (2.0 * std::f64::consts::PI * 48.0 * t).sin() * 0.25;
                    drone * (0.8 + 0.2 * beat_decay)
                }
                BgmType::FunnyMeme => {
                    // 滑稽跳跃调频音
                    let freq = 180.0 + (beat_pos as f64 / beat_samples as f64) * 80.0;
                    (2.0 * std::f64::consts::PI * freq * t).sin() * beat_decay * 0.4
                }
                BgmType::None => 0.0,
            };

            let sample = (sample_val * 0.4 * (i16::MAX as f64)) as i16;
            writer.write_sample(sample).map_err(|e| format!("写入 BGM 采样失败: {}", e))?;
        }

        writer.finalize().map_err(|e| format!("写入 BGM 结束失败: {}", e))?;
        Ok(output_path.to_path_buf())
    }
}
