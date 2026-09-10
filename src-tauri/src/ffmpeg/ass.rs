use std::path::Path;
use crate::ffmpeg::pipeline::SubtitleCue;

/// ASS 动效字幕生成器 (Advanced SubStation Alpha v4.00+)
pub struct AssSubtitleGenerator;

impl AssSubtitleGenerator {
    /// 格式化 ASS 时间戳: h:mm:ss.cc (厘秒)
    pub fn format_ass_time(seconds: f64) -> String {
        let total_cs = (seconds * 100.0).round() as u64; // 厘秒 (1/100s)
        let cs = total_cs % 100;
        let total_sec = total_cs / 100;
        let s = total_sec % 60;
        let total_min = total_sec / 60;
        let m = total_min % 60;
        let h = total_min / 60;
        format!("{}:{:02}:{:02}.{:02}", h, m, s, cs)
    }

    /// 生成带有卡拉OK动效与关键词高亮的 ASS 脚本内容
    /// 参数：
    /// - `cues`: 字幕切分条目
    /// - `is_vertical`: 是否为 9:16 竖屏 (True: 1080x1920, False: 1920x1080)
    pub fn generate_ass_script(cues: &[SubtitleCue], is_vertical: bool) -> String {
        let (res_x, res_y, font_size, margin_v) = if is_vertical {
            (1080, 1920, 58, 380) // 竖屏底边抬高 380px，避开抖音/TikTok底部标题栏
        } else {
            (1920, 1080, 48, 80)
        };

        let mut content = String::new();

        // 1. Script Info 头部
        content.push_str("[Script Info]\n");
        content.push_str("Title: NeuroClip Kinetic Pop-up Subtitles\n");
        content.push_str("ScriptType: v4.00+\n");
        content.push_str("WrapStyle: 0\n");
        content.push_str("ScaledBorderAndShadow: yes\n");
        content.push_str(&format!("PlayResX: {}\n", res_x));
        content.push_str(&format!("PlayResY: {}\n\n", res_y));

        // 2. V4+ Styles 样式表 (包含默认白字粗描边样式与爆款黄色强调样式)
        content.push_str("[V4+ Styles]\n");
        content.push_str("Format: Name, Fontname, Fontsize, PrimaryColour, SecondaryColour, OutlineColour, BackColour, Bold, Italic, Underline, StrikeOut, ScaleX, ScaleY, Spacing, Angle, BorderStyle, Outline, Shadow, Alignment, MarginL, MarginR, MarginV, Encoding\n");
        
        // Default Style: 粗白字, 黑色加粗外描边 (Outline 4.5), 投影 3.0, 底部居中对齐 (Alignment 2)
        content.push_str(&format!(
            "Style: Default,Arial,{},&H00FFFFFF,&H0000FFFF,&H00000000,&H80000000,-1,0,0,0,100,100,1,0,1,4.5,2.5,2,40,40,{},1\n",
            font_size, margin_v
        ));
        
        // Pop Style: 亮青色/高光加粗跳动强调样式
        content.push_str(&format!(
            "Style: Highlight,Arial,{},&H0000F0FF,&H0000FFFF,&H00000000,&H90000000,-1,0,0,0,112,112,2,0,1,5.5,3.5,2,40,40,{},1\n\n",
            (font_size as f32 * 1.1) as u32, margin_v
        ));

        // 3. Events 事件对话流
        content.push_str("[Events]\n");
        content.push_str("Format: Layer, Start, End, Style, Name, MarginL, MarginR, MarginV, Effect, Text\n");

        let keywords = [
            "注意看", "千万别眨眼", "谁能想到", "神级名场面", "反杀", "千钧一发",
            "翻盘", "封神", "不可思议", "全场起立", "绝杀", "细节", "巅峰"
        ];

        for cue in cues {
            let start = Self::format_ass_time(cue.start_sec);
            let end = Self::format_ass_time(cue.end_sec);
            let char_count = cue.text.chars().count().max(1);
            let total_dur_cs = ((cue.end_sec - cue.start_sec).max(0.2) * 100.0) as u32;
            let cs_per_char = (total_dur_cs / char_count as u32).max(5);

            // 检查是否包含核心爆款关键词
            let has_keyword = keywords.iter().any(|k| cue.text.contains(k));

            let mut styled_text = String::new();

            if has_keyword {
                // 若包含关键词，整体施加跳动放大动效与荧光黄色泽
                styled_text.push_str("{\\c&H00E5FF&\\t(0,180,\\fscx115\\fscy115)\\t(180,360,\\fscx100\\fscy100)}");
                styled_text.push_str(&cue.text);
            } else {
                // 逐字卡拉OK跳动标签
                for c in cue.text.chars() {
                    styled_text.push_str(&format!("{{\\k{}}}{}", cs_per_char, c));
                }
            }

            let style_name = if has_keyword { "Highlight" } else { "Default" };
            content.push_str(&format!(
                "Dialogue: 0,{},{},{},,0,0,0,,{}\n",
                start, end, style_name, styled_text
            ));
        }

        content
    }

    /// 将 ASS 脚本写入指定文件
    pub fn write_ass_file(cues: &[SubtitleCue], is_vertical: bool, output_path: &Path) -> Result<(), String> {
        if let Some(parent) = output_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        let script = Self::generate_ass_script(cues, is_vertical);
        std::fs::write(output_path, script).map_err(|e| format!("写入 ASS 字幕文件失败: {}", e))
    }
}
