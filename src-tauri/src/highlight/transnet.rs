use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShotTransition {
    pub time_sec: f64,
    pub frame_idx: u64,
    pub confidence: f32,
}

pub struct TransNetSnapper {
    pub snap_window_sec: f64,   // 寻找最近切点的吸附窗口半径 (例如 3.0s)
    pub min_duration_sec: f64,  // 最小片段时长 (15.0s)
    pub max_duration_sec: f64,  // 最大片段时长 (45.0s)
}

impl Default for TransNetSnapper {
    fn default() -> Self {
        Self {
            snap_window_sec: 3.0,
            min_duration_sec: 15.0,
            max_duration_sec: 45.0,
        }
    }
}

impl TransNetSnapper {
    pub fn new(snap_window_sec: f64, min_duration_sec: f64, max_duration_sec: f64) -> Self {
        Self {
            snap_window_sec,
            min_duration_sec,
            max_duration_sec,
        }
    }

    /// 将候选起止时间吸附至最近的无破损镜头边缘切点
    /// 保证不粗暴打断讲话或画面运镜，并严格约束在 15s~45s 黄金短视频区间
    pub fn snap_interval(
        &self,
        raw_start: f64,
        raw_end: f64,
        transitions: &[ShotTransition],
        video_duration: f64,
    ) -> (f64, f64, bool, bool) {
        let cuts: Vec<f64> = transitions.iter().map(|t| t.time_sec).collect();

        // 1. 吸附开始时间点
        let (snapped_start, start_snapped) = self.find_nearest_cut(raw_start, &cuts);

        // 2. 吸附结束时间点
        let (snapped_end, end_snapped) = self.find_nearest_cut(raw_end, &cuts);

        // 3. 边界与时长校验 (15s ~ 45s)
        let mut final_start = snapped_start.max(0.0);
        let mut final_end = snapped_end.min(video_duration);

        let current_dur = final_end - final_start;

        if current_dur < self.min_duration_sec {
            // 时长不足 15s，尝试向后寻找下一个镜头切点
            let needed = self.min_duration_sec - current_dur;
            let target_end = final_end + needed;
            let (expanded_end, _) = self.find_nearest_cut(target_end, &cuts);
            final_end = expanded_end.min(video_duration);

            // 若依然不足，直接强制补足至 min_duration
            if final_end - final_start < self.min_duration_sec {
                final_end = (final_start + self.min_duration_sec).min(video_duration);
                if final_end - final_start < self.min_duration_sec {
                    final_start = (final_end - self.min_duration_sec).max(0.0);
                }
            }
        } else if current_dur > self.max_duration_sec {
            // 时长超过 45s，寻找内部最近切点压缩
            let target_end = final_start + self.max_duration_sec;
            let (clamped_end, _) = self.find_nearest_cut(target_end, &cuts);
            if clamped_end > final_start + self.min_duration_sec && clamped_end <= final_start + self.max_duration_sec {
                final_end = clamped_end;
            } else {
                final_end = final_start + self.max_duration_sec;
            }
        }

        (final_start, final_end, start_snapped, end_snapped)
    }

    /// 在 snap_window_sec 半径内寻找最近的镜头 Cut 点
    fn find_nearest_cut(&self, target_time: f64, cuts: &[f64]) -> (f64, bool) {
        let mut best_cut = target_time;
        let mut min_diff = self.snap_window_sec;
        let mut found = false;

        for &cut in cuts {
            let diff = (cut - target_time).abs();
            if diff < min_diff {
                min_diff = diff;
                best_cut = cut;
                found = true;
            }
        }

        (best_cut, found)
    }
}
