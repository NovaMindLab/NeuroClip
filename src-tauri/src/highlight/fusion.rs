use serde::{Deserialize, Serialize};
use crate::highlight::rms::AudioRmsAnalyzer;
use crate::highlight::sound_events::YamNetDetector;
use crate::highlight::transnet::{ShotTransition, TransNetSnapper};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HighlightSegment {
    pub id: usize,
    pub start_time: f64,
    pub end_time: f64,
    pub duration: f64,
    pub score: f32,
    pub burst_ratio: f32,
    pub emotion_tags: Vec<String>,
    pub start_snapped: bool,
    pub end_snapped: bool,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultimodalAnalysisResult {
    pub total_duration: f64,
    pub rms_curve_sample: Vec<(f64, f32)>, // 采样供前端波形展示
    pub candidates: Vec<HighlightSegment>,
    pub transitions_count: usize,
}

pub struct MultimodalFusionEngine {
    pub rms_analyzer: AudioRmsAnalyzer,
    pub yamnet_detector: YamNetDetector,
    pub snapper: TransNetSnapper,
    pub max_clips: usize,
}

impl Default for MultimodalFusionEngine {
    fn default() -> Self {
        Self {
            rms_analyzer: AudioRmsAnalyzer::default(),
            yamnet_detector: YamNetDetector::default(),
            snapper: TransNetSnapper::default(),
            max_clips: 3,
        }
    }
}

impl MultimodalFusionEngine {
    pub fn new(max_clips: usize) -> Self {
        Self {
            max_clips,
            ..Default::default()
        }
    }

    /// 双通道多模态高光决策执行
    pub fn fuse(
        &self,
        samples: &[f32],
        sample_rate: u32,
        transitions: &[ShotTransition],
        video_duration: f64,
    ) -> Result<MultimodalAnalysisResult, String> {
        // 1. 通道 A: 计算音频滑窗 RMS 与能量激增区间
        let (curve, energy_intervals) = self.rms_analyzer.analyze_samples(samples, sample_rate)?;

        // 2. 通道 A: 情绪音效标签检测 (YAMNet)
        let sound_events = self.yamnet_detector.detect_events_from_curve(&curve, &energy_intervals);

        // 3. 通道 B: 结合视觉镜头切点，吸附候选区间并计算综合加权评分
        let mut scored_segments: Vec<HighlightSegment> = Vec::new();

        for (idx, interval) in energy_intervals.iter().enumerate() {
            let (snapped_start, snapped_end, start_snapped, end_snapped) = self.snapper.snap_interval(
                interval.start_sec,
                interval.end_sec,
                transitions,
                video_duration,
            );

            let (emotion_score, detected_types) = self.yamnet_detector.score_interval(
                &sound_events,
                snapped_start,
                snapped_end,
            );

            // 归一化综合评分: 能量突发占比 (0.5) + 情绪事件加权 (0.3) + 镜头切点契合度 (0.2)
            let burst_norm = (interval.burst_ratio / 5.0).min(1.0);
            let snap_bonus = if start_snapped && end_snapped { 1.0 } else if start_snapped || end_snapped { 0.6 } else { 0.2 };
            let composite_score = (burst_norm * 0.5) + (emotion_score.min(1.0) * 0.3) + (snap_bonus * 0.2);

            let tag_strings: Vec<String> = detected_types.iter().map(|t| t.label_zh().to_string()).collect();
            let summary = format!(
                "高光 #{}: 能量飙升 {:.1}x，检测到[{}]，时长 {:.1}秒",
                idx + 1,
                interval.burst_ratio,
                if tag_strings.is_empty() { "高能时刻".to_string() } else { tag_strings.join(", ") },
                snapped_end - snapped_start
            );

            scored_segments.push(HighlightSegment {
                id: idx + 1,
                start_time: (snapped_start * 100.0).round() / 100.0,
                end_time: (snapped_end * 100.0).round() / 100.0,
                duration: ((snapped_end - snapped_start) * 100.0).round() / 100.0,
                score: (composite_score * 100.0).round() / 100.0,
                burst_ratio: (interval.burst_ratio * 10.0).round() / 10.0,
                emotion_tags: tag_strings,
                start_snapped,
                end_snapped,
                summary,
            });
        }

        // 4. 去重与重叠区间合并：如果有重叠超过 50% 的片段，保留评分最高者
        scored_segments.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
        let mut final_candidates: Vec<HighlightSegment> = Vec::new();

        for seg in scored_segments {
            let mut overlapped = false;
            for existing in &final_candidates {
                let overlap_start = seg.start_time.max(existing.start_time);
                let overlap_end = seg.end_time.min(existing.end_time);
                if overlap_end > overlap_start {
                    let overlap_len = overlap_end - overlap_start;
                    if overlap_len > (seg.duration.min(existing.duration) * 0.4) {
                        overlapped = true;
                        break;
                    }
                }
            }
            if !overlapped {
                final_candidates.push(seg);
            }
            if final_candidates.len() >= self.max_clips {
                break;
            }
        }

        // 按照起始时间重新升序排列
        final_candidates.sort_by(|a, b| a.start_time.partial_cmp(&b.start_time).unwrap());
        for (i, c) in final_candidates.iter_mut().enumerate() {
            c.id = i + 1;
        }

        // 降采样 curve 供前端波形组件直接渲染 (最多保留 300 点)
        let sample_step = (curve.len() / 300).max(1);
        let sampled_curve: Vec<(f64, f32)> = curve.into_iter().step_by(sample_step).collect();

        Ok(MultimodalAnalysisResult {
            total_duration: video_duration,
            rms_curve_sample: sampled_curve,
            candidates: final_candidates,
            transitions_count: transitions.len(),
        })
    }
}
