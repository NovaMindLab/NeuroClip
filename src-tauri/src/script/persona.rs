use serde::{Deserialize, Serialize};
use crate::ffmpeg::pipeline::SubtitleCue;
use crate::script::prompt::CommentaryScript;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommentaryPersona {
    SuspenseDrama,     // 悬念反转风 (3.8字/秒)
    HighEnergyEsports, // 激情电竞风 (5.0字/秒)
    HumorousRoast,     // 幽默下饭风 (4.0字/秒)
    TacticalBreakdown, // 硬核科普风 (4.2字/秒)
}

impl Default for CommentaryPersona {
    fn default() -> Self {
        CommentaryPersona::HighEnergyEsports
    }
}

impl CommentaryPersona {
    pub fn name_zh(&self) -> &'static str {
        match self {
            CommentaryPersona::SuspenseDrama => "悬念反转风",
            CommentaryPersona::HighEnergyEsports => "激情电竞风",
            CommentaryPersona::HumorousRoast => "幽默下饭风",
            CommentaryPersona::TacticalBreakdown => "硬核科普风",
        }
    }

    /// 该流派的目标中文语速 (字/秒)
    pub fn char_rate(&self) -> f64 {
        match self {
            CommentaryPersona::SuspenseDrama => 3.8,
            CommentaryPersona::HighEnergyEsports => 5.0,
            CommentaryPersona::HumorousRoast => 4.0,
            CommentaryPersona::TacticalBreakdown => 4.2,
        }
    }

    /// 针对流派生成前 3 秒黄金 Hook
    pub fn generate_hook(&self, topic: &str) -> String {
        match self {
            CommentaryPersona::SuspenseDrama => {
                format!("注意看！谁能想到原本死局的{}，竟埋下了惊天伏笔！", topic)
            }
            CommentaryPersona::HighEnergyEsports => {
                format!("千万别眨眼！这波{}神级操作直接把全场看傻了！", topic)
            }
            CommentaryPersona::HumorousRoast => {
                format!("原谅我不厚道地笑了！这波{}操作直接承包整晚笑点！", topic)
            }
            CommentaryPersona::TacticalBreakdown => {
                format!("90%的人都没看懂！放慢这波{}的细节，堪称教科书！", topic)
            }
        }
    }

    /// 根据流派与目标时长生成严格配速的文案和分句字幕
    pub fn generate_script(
        &self,
        duration_sec: f64,
        emotion_tags: &[String],
        topic_hint: Option<&str>,
    ) -> CommentaryScript {
        let rate = self.char_rate();
        let target_chars = (duration_sec * rate).round() as usize;
        let topic = topic_hint.unwrap_or("决胜时刻");
        let hook = self.generate_hook(topic);

        let mid_text = match self {
            CommentaryPersona::SuspenseDrama => format!(
                "就在所有人都以为局势已定时，关键细节悄然逆转，呼吸之间胜负彻底颠覆，让人不得不倒吸一口凉气！"
            ),
            CommentaryPersona::HighEnergyEsports => format!(
                "行云流水的极限走位，每一个技能都精准卡在毫厘之间，伤害瞬间拉满，全场观众集体起立陷入狂欢，这波操作直接封神！"
            ),
            CommentaryPersona::HumorousRoast => format!(
                "本以为是个王者降临，没想到反手就是一个意料之外的神级下饭名场面，现场解说都差点没绷住，简直太魔性了！"
            ),
            CommentaryPersona::TacticalBreakdown => format!(
                "注意看这零点一秒的卡视野，极致的心理博弈配合果断的决策执行，完美诠释了顶尖职业选手的绝对嗅觉！"
            ),
        };

        let tag_hint = if emotion_tags.contains(&"欢呼声".to_string()) {
            "现场瞬间彻底沸腾！"
        } else {
            "这一刻注定成为名场面！"
        };

        let mut raw_text = format!("{}{}{}", hook, mid_text, tag_hint);

        // 字符数微调逼近目标配速 (误差控制在 ±4 字)
        let current_len = raw_text.chars().count();
        if current_len > target_chars + 3 {
            let truncated: String = raw_text.chars().take(target_chars).collect();
            raw_text = format!("{}！", truncated.trim_end_matches(['！', '。', '，']));
        } else if current_len < target_chars.saturating_sub(3) {
            let padding = "不得不说这绝对是巅峰时刻！";
            raw_text.push_str(padding);
            let truncated: String = raw_text.chars().take(target_chars).collect();
            raw_text = format!("{}！", truncated.trim_end_matches(['！', '。', '，']));
        }

        // 分句与时间轴切分 (前 3 秒严格分配给黄金 Hook)
        let sentences: Vec<&str> = raw_text.split(['！', '。', '，', '、'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        let total_chars: usize = sentences.iter().map(|s| s.chars().count()).sum();
        let mut cues = Vec::new();
        let mut curr_time = 0.0;

        for (i, &s) in sentences.iter().enumerate() {
            let char_len = s.chars().count();
            let seg_duration = if i == 0 {
                3.0 // 黄金 Hook 3.0s
            } else {
                let remaining_time = (duration_sec - 3.0).max(1.0);
                let remaining_chars = total_chars.saturating_sub(sentences[0].chars().count()).max(1);
                (char_len as f64 / remaining_chars as f64) * remaining_time
            };

            let end_time = (curr_time + seg_duration).min(duration_sec);
            cues.push(SubtitleCue {
                start_sec: (curr_time * 100.0).round() / 100.0,
                end_sec: (end_time * 100.0).round() / 100.0,
                text: s.to_string(),
            });
            curr_time = end_time;
        }

        let char_count = raw_text.chars().count();
        CommentaryScript {
            hook,
            full_commentary: raw_text,
            duration_seconds: duration_sec,
            char_count,
            subtitles: cues,
        }
    }
}
