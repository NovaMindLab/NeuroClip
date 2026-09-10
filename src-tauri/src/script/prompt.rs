use serde::{Deserialize, Serialize};
use crate::ffmpeg::pipeline::SubtitleCue;

/// 生产级短视频解说 System Prompt
pub const NEUROCLIP_COMMENTARY_SYSTEM_PROMPT: &str = r#"你是一位百万粉短视频爆款解说文案大师。
你的核心任务是：针对给定的视频高光片段（包含场景背景、情绪标签、时长），撰写极具抓人眼球、节奏紧凑的高能解说词。

【硬性约束条件（违反将导致系统解析崩溃）】：
1. 【前 3 秒黄金悬念 Hook】：第一句话必须是 3 秒内吸睛的爆款 Hook（例如："注意看！就在这一瞬间"、"谁能想到，这波操作直接封神"、"千钧一发之际，奇迹发生了"、"Watch closely! In this split second..."）。
2. 【字数严格时空对齐】：中文配音语速标准为 4.2 字/秒。生成的总字符数（不含标点符号）必须严格对齐：
   目标字数 = Math.round(片段时长秒数 * 4.2)
   允许误差范围在 ±3 个字以内。绝对不能超时或大幅不足！
3. 【标准化纯 JSON 输出】：严禁输出任何 Markdown 标记（如 ```json）、严禁输出任何前言、后记或解释性文字。仅输出符合以下 JSON Schema 的标准化数据格式：

{
  "hook": "前3秒爆点文案",
  "full_commentary": "完整解说文案全文",
  "duration_seconds": 25.5,
  "char_count": 107,
  "subtitles": [
    {
      "start_sec": 0.0,
      "end_sec": 3.0,
      "text": "前3秒爆点文案"
    },
    {
      "start_sec": 3.0,
      "end_sec": 7.5,
      "text": "第二句文案"
    }
  ]
}
"#;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentaryScript {
    pub hook: String,
    pub full_commentary: String,
    pub duration_seconds: f64,
    pub char_count: usize,
    pub subtitles: Vec<SubtitleCue>,
}

pub struct CommentaryGenerator;

impl CommentaryGenerator {
    /// 计算目标字数 (按 4.2 字/秒标准)
    pub fn target_char_count(duration_sec: f64) -> usize {
        (duration_sec * 4.2).round() as usize
    }

    /// 格式化用于发送给大模型/AI 的上下文 Prompt
    pub fn build_user_prompt(
        clip_summary: &str,
        duration_sec: f64,
        emotion_tags: &[String],
    ) -> String {
        let target_chars = Self::target_char_count(duration_sec);
        let tags_str = if emotion_tags.is_empty() {
            "高能瞬间".to_string()
        } else {
            emotion_tags.join("、")
        };

        format!(
            "【高光片段信息】\n- 场景概要: {}\n- 情绪识别标签: [{}]\n- 片段精准时长: {:.1} 秒\n- 目标严格字数: 约 {} 字（按 4.2字/秒配速）\n\n请立即为该片段生成带黄金Hook的解说文案与字幕时间轴 JSON：",
            clip_summary, tags_str, duration_sec, target_chars
        )
    }

    /// 内置启发式爆款文案生成器 (在离线无网络 API Key 时提供高质量爆款模板渲染)
    pub fn generate_heuristic_script(
        duration_sec: f64,
        emotion_tags: &[String],
        topic_hint: Option<&str>,
    ) -> CommentaryScript {
        let target_chars = Self::target_char_count(duration_sec);
        let tag = emotion_tags.first().map(|s| s.as_str()).unwrap_or("热烈气氛");
        let topic = topic_hint.unwrap_or("全场焦点");

        // 构造前 3 秒黄金 Hook
        let hooks = [
            "注意看！就在这千钧一发的瞬间！",
            "谁能想到！接下来这一幕彻底引爆全场！",
            "千万别眨眼！这波操作堪称神级名场面！",
            "注意看！现场所有人都屏住了呼吸！",
        ];
        let hook = hooks[(duration_sec as usize) % hooks.len()].to_string();

        let mid_text = match tag {
            "欢呼声" | "热烈掌声" => format!(
                "{}突然发力，以不可思议的绝妙节奏瞬间扭转战局，行云流水般的掌控力让全场观众集体起立狂欢！",
                topic
            ),
            "大笑" => format!(
                "{}突然上演意料之外的幽默反转，现场气氛直接拉满，这波神来之笔让所有人忍俊不禁！",
                topic
            ),
            _ => format!(
                "{}展现出顶级的反应与爆发力，每一个细节都拿捏得恰到好处，堪称教科书级别的巅峰演绎！",
                topic
            ),
        };

        let ending = "不得不说，这一刻的精彩注定成为经典回放！";

        let mut raw_text = format!("{}{}{}", hook, mid_text, ending);

        // 微调字数逼近 target_chars
        let current_len = raw_text.chars().count();
        if current_len > target_chars + 3 {
            let truncated: String = raw_text.chars().take(target_chars).collect();
            raw_text = format!("{}！", truncated.trim_end_matches(['！', '。', '，']));
        } else if current_len < target_chars.saturating_sub(3) {
            let padding = "这绝对是不可多得的高光时刻！";
            raw_text.push_str(padding);
            let truncated: String = raw_text.chars().take(target_chars).collect();
            raw_text = format!("{}！", truncated.trim_end_matches(['！', '。', '，']));
        }

        // 分句切分字幕时间轴
        let sentences: Vec<&str> = raw_text.split(['！', '。', '，', '、'])
            .filter(|s| !s.trim().is_empty())
            .collect();

        let total_chars: usize = sentences.iter().map(|s| s.chars().count()).sum();
        let mut cues = Vec::new();
        let mut curr_time = 0.0;

        for (i, &s) in sentences.iter().enumerate() {
            let char_len = s.chars().count();
            let seg_duration = if i == 0 {
                3.0 // 黄金 Hook 严格占 3 秒
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

        let final_count = raw_text.chars().count();
        CommentaryScript {
            hook,
            full_commentary: raw_text,
            duration_seconds: duration_sec,
            char_count: final_count,
            subtitles: cues,
        }
    }
}
