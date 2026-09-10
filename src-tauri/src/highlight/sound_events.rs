use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum SoundEventType {
    Cheering,   // 欢呼
    Laughter,   // 大笑
    Screaming,  // 尖叫
    Applause,   // 掌声
    Speech,     // 说话
    Other,      // 其他
}

impl SoundEventType {
    pub fn label_zh(&self) -> &'static str {
        match self {
            SoundEventType::Cheering => "欢呼声",
            SoundEventType::Laughter => "大笑",
            SoundEventType::Screaming => "尖叫/呐喊",
            SoundEventType::Applause => "热烈掌声",
            SoundEventType::Speech => "人声叙述",
            SoundEventType::Other => "环境声",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoundEventCue {
    pub time_sec: f64,
    pub event_type: SoundEventType,
    pub confidence: f32,
}

pub struct YamNetDetector {
    pub min_confidence: f32,
}

impl Default for YamNetDetector {
    fn default() -> Self {
        Self { min_confidence: 0.35 }
    }
}

impl YamNetDetector {
    pub fn new(min_confidence: f32) -> Self {
        Self { min_confidence }
    }

    /// 计算指定时间范围内的情绪声效加权得分
    /// 欢呼、掌声、大笑、尖叫等情绪事件拥有高权重
    pub fn score_interval(&self, events: &[SoundEventCue], start_sec: f64, end_sec: f64) -> (f32, Vec<SoundEventType>) {
        let mut score = 0.0f32;
        let mut detected_types = Vec::new();

        for ev in events {
            if ev.time_sec >= start_sec && ev.time_sec <= end_sec && ev.confidence >= self.min_confidence {
                let weight = match ev.event_type {
                    SoundEventType::Cheering => 1.5,
                    SoundEventType::Screaming => 1.4,
                    SoundEventType::Applause => 1.2,
                    SoundEventType::Laughter => 1.1,
                    SoundEventType::Speech => 0.4,
                    SoundEventType::Other => 0.1,
                };
                score += ev.confidence * weight;
                if !detected_types.contains(&ev.event_type) {
                    detected_types.push(ev.event_type.clone());
                }
            }
        }

        (score, detected_types)
    }

    /// 启发式/声学特征自适应事件探测器 (用于离线与无独立权重时的稳健运行)
    pub fn detect_events_from_curve(&self, _curve: &[(f64, f32)], burst_intervals: &[crate::highlight::rms::EnergyInterval]) -> Vec<SoundEventCue> {
        let mut events = Vec::new();
        for item in burst_intervals {
            // 根据突增比率与时长分类最可能的声音事件
            let ev_type = if item.burst_ratio > 4.0 {
                SoundEventType::Cheering
            } else if item.burst_ratio > 3.0 {
                SoundEventType::Applause
            } else if item.end_sec - item.start_sec > 3.0 {
                SoundEventType::Laughter
            } else {
                SoundEventType::Screaming
            };

            let conf = ((item.burst_ratio / 5.0).min(0.95)).max(0.4);
            events.push(SoundEventCue {
                time_sec: (item.start_sec + item.end_sec) / 2.0,
                event_type: ev_type,
                confidence: conf,
            });
        }
        events
    }
}
