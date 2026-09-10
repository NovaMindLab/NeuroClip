use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyInterval {
    pub start_sec: f64,
    pub end_sec: f64,
    pub peak_rms: f32,
    pub avg_rms: f32,
    pub burst_ratio: f32,
}

pub struct AudioRmsAnalyzer {
    pub sample_rate: u32,
    pub window_sec: f64,
    pub hop_sec: f64,
    pub burst_factor: f32, // 默认 2.5 倍
}

impl Default for AudioRmsAnalyzer {
    fn default() -> Self {
        Self {
            sample_rate: 16000,
            window_sec: 1.0,
            hop_sec: 0.5,
            burst_factor: 2.5,
        }
    }
}

impl AudioRmsAnalyzer {
    pub fn new(window_sec: f64, hop_sec: f64, burst_factor: f32) -> Self {
        Self {
            sample_rate: 16000,
            window_sec,
            hop_sec,
            burst_factor,
        }
    }

    /// 从 16kHz WAV 读取单声道 PCM 样本并计算短时滑窗 RMS 曲线
    pub fn analyze_wav_file(&self, wav_path: &Path) -> Result<(Vec<(f64, f32)>, Vec<EnergyInterval>), String> {
        let mut reader = hound::WavReader::open(wav_path)
            .map_err(|e| format!("无法打开 WAV 音频文件 {:?}: {}", wav_path, e))?;

        let spec = reader.spec();
        let samples: Vec<f32> = match spec.sample_format {
            hound::SampleFormat::Int => {
                let max_val = (1i64 << (spec.bits_per_sample - 1)) as f32;
                reader.samples::<i32>()
                    .filter_map(|s| s.ok())
                    .map(|s| (s as f32) / max_val)
                    .collect()
            }
            hound::SampleFormat::Float => {
                reader.samples::<f32>().filter_map(|s| s.ok()).collect()
            }
        };

        if samples.is_empty() {
            return Err("音频样本为空".to_string());
        }

        self.analyze_samples(&samples, spec.sample_rate)
    }

    /// 基于浮点样本数组执行滑窗能量与爆点提取
    pub fn analyze_samples(&self, samples: &[f32], sample_rate: u32) -> Result<(Vec<(f64, f32)>, Vec<EnergyInterval>), String> {
        let window_size = ((self.window_sec * sample_rate as f64) as usize).max(1);
        let hop_size = ((self.hop_sec * sample_rate as f64) as usize).max(1);

        let mut curve: Vec<(f64, f32)> = Vec::new();
        let mut total_rms_sum = 0.0f64;
        let mut total_windows = 0;

        let mut offset = 0;
        while offset + window_size <= samples.len() {
            let window = &samples[offset..offset + window_size];
            let sum_sq: f64 = window.iter().map(|&x| (x as f64) * (x as f64)).sum();
            let rms = (sum_sq / window_size as f64).sqrt() as f32;
            let time_sec = (offset as f64) / (sample_rate as f64);

            curve.push((time_sec, rms));
            total_rms_sum += rms as f64;
            total_windows += 1;

            offset += hop_size;
        }

        if total_windows == 0 {
            return Ok((curve, Vec::new()));
        }

        let mean_rms = (total_rms_sum / total_windows as f64) as f32;
        // 动态自适应阈值，避免极安静音频微弱杂音被过度放大
        let baseline_rms = mean_rms.max(0.015);
        let threshold = baseline_rms * self.burst_factor;

        // 抓取高于阈值的区间并合并邻近窗口 (聚类间隔 2.5 秒以内)
        let mut intervals: Vec<EnergyInterval> = Vec::new();
        let mut current_start: Option<f64> = None;
        let mut current_end = 0.0;
        let mut current_peak = 0.0f32;
        let mut current_sum = 0.0f64;
        let mut current_count = 0;

        for &(t, rms) in &curve {
            if rms >= threshold {
                match current_start {
                    None => {
                        current_start = Some(t);
                        current_end = t + self.window_sec;
                        current_peak = rms;
                        current_sum = rms as f64;
                        current_count = 1;
                    }
                    Some(_) => {
                        current_end = t + self.window_sec;
                        if rms > current_peak {
                            current_peak = rms;
                        }
                        current_sum += rms as f64;
                        current_count += 1;
                    }
                }
            } else if let Some(start) = current_start {
                // 如果当前距离区间结束超过 2.5 秒，则封口结算
                if t - current_end > 2.5 {
                    let avg = (current_sum / current_count.max(1) as f64) as f32;
                    intervals.push(EnergyInterval {
                        start_sec: start,
                        end_sec: current_end,
                        peak_rms: current_peak,
                        avg_rms: avg,
                        burst_ratio: current_peak / baseline_rms,
                    });
                    current_start = None;
                }
            }
        }

        // 处理末尾区间
        if let Some(start) = current_start {
            let avg = (current_sum / current_count.max(1) as f64) as f32;
            intervals.push(EnergyInterval {
                start_sec: start,
                end_sec: current_end,
                peak_rms: current_peak,
                avg_rms: avg,
                burst_ratio: current_peak / baseline_rms,
            });
        }

        Ok((curve, intervals))
    }
}
