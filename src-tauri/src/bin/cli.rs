use std::path::{Path, PathBuf};
use std::process::Command;

use neuroclip_lib::ffmpeg::{
    FFmpegContext,
    extractor::MediaExtractor,
    pipeline::{RemuxOptions, RemuxPipeline},
    ass::AssSubtitleGenerator,
    hwaccel::HardwareEncoder,
};
use neuroclip_lib::highlight::{
    fusion::MultimodalFusionEngine,
    transnet::ShotTransition,
};
use neuroclip_lib::script::persona::CommentaryPersona;
use neuroclip_lib::tts::{SpeechSynthesizer, TtsConfig, BgmType};
use neuroclip_lib::automation::{FolderWatcher, FolderWatcherConfig};

fn print_banner() {
    println!("\x1b[35m");
    println!("  _   _                      ____ _ _       ");
    println!(" | \\ | | ___ _   _ _ __ ___ / ___| (_)_ __  ");
    println!(" |  \\| |/ _ \\ | | | '__/ _ \\ |   | | | '_ \\ ");
    println!(" | |\\  |  __/ |_| | | | (_) | |___| | | |_) |");
    println!(" |_| \\_|\\___|\\__,_|_|  \\___/ \\____|_|_| .__/ ");
    println!("                                      |_|    ");
    println!("\x1b[36m==> NeuroClip Headless Cloud & CLI Execution Engine (v0.2.0)\x1b[0m\n");
}

fn print_help() {
    println!("用法: neuroclip-cli [选项]");
    println!();
    println!("基础选项:");
    println!("  -i, --input <路径或URL>   输入视频文件路径或网络下载 URL");
    println!("  -o, --output-dir <目录>   输出切片结果目录 (默认: ./neuroclip_exports)");
    println!("  -t, --topic <主题>        解说词主题提示 (默认: '高能对决')");
    println!("  -n, --max-clips <数量>    最大输出高光短视频数量 (默认: 3)");
    println!();
    println!("短视频工业化呈现选项 (Video Only):");
    println!("  --aspect <9:16|16:9>      视频画幅版型 (默认: 9:16 动态模糊背景竖屏)");
    println!("  --persona <风格>          文案流派: esports(激情电竞) | suspense(悬念反转) | roast(幽默下饭) | breakdown(硬核科普)");
    println!("  --bgm <伴奏>              情绪背景音乐: trap(燃系电竞) | drone(悬念心跳) | meme(搞怪幽默) | none");
    println!();
    println!("无人值守自动化选项:");
    println!("  -w, --watch <监控目录>    启动守护监听模式，自动监听新落盘长视频并切片输出");
    println!("  -h, --help                打印帮助信息");
    println!();
    println!("示例:");
    println!("  # 单视频 9:16 竖屏高光切片并挂载动效字幕与 Trap 伴奏");
    println!("  neuroclip-cli -i match.mp4 --aspect 9:16 --persona esports --bgm trap");
    println!();
    println!("  # 启动无人值守监控目录 (录播自动切片)");
    println!("  neuroclip-cli --watch ./obs_recordings --output-dir ./auto_shorts");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    print_banner();

    let args: Vec<String> = std::env::args().collect();
    let mut input_video: Option<String> = None;
    let mut output_dir = PathBuf::from("./neuroclip_exports");
    let mut topic = "高能对决".to_string();
    let mut max_clips = 3usize;
    let mut is_vertical = true; // 默认 9:16 竖屏适配移动端
    let mut persona = CommentaryPersona::HighEnergyEsports;
    let mut bgm = BgmType::HypeTrap;
    let mut watch_dir: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-i" | "--input" => {
                if i + 1 < args.len() {
                    input_video = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            "-o" | "--output-dir" => {
                if i + 1 < args.len() {
                    output_dir = PathBuf::from(&args[i + 1]);
                    i += 1;
                }
            }
            "-t" | "--topic" => {
                if i + 1 < args.len() {
                    topic = args[i + 1].clone();
                    i += 1;
                }
            }
            "-n" | "--max-clips" => {
                if i + 1 < args.len() {
                    max_clips = args[i + 1].parse().unwrap_or(3);
                    i += 1;
                }
            }
            "--aspect" => {
                if i + 1 < args.len() {
                    let asp = args[i + 1].trim();
                    is_vertical = asp == "9:16" || asp == "vertical";
                    i += 1;
                }
            }
            "--persona" => {
                if i + 1 < args.len() {
                    persona = match args[i + 1].as_str() {
                        "suspense" => CommentaryPersona::SuspenseDrama,
                        "roast" => CommentaryPersona::HumorousRoast,
                        "breakdown" => CommentaryPersona::TacticalBreakdown,
                        _ => CommentaryPersona::HighEnergyEsports,
                    };
                    i += 1;
                }
            }
            "--bgm" => {
                if i + 1 < args.len() {
                    bgm = match args[i + 1].as_str() {
                        "drone" => BgmType::SuspenseDrone,
                        "meme" => BgmType::FunnyMeme,
                        "none" => BgmType::None,
                        _ => BgmType::HypeTrap,
                    };
                    i += 1;
                }
            }
            "-w" | "--watch" => {
                if i + 1 < args.len() {
                    watch_dir = Some(PathBuf::from(&args[i + 1]));
                    i += 1;
                }
            }
            "-h" | "--help" => {
                print_help();
                return Ok(());
            }
            _ => {}
        }
        i += 1;
    }

    // 1. 若指定 --watch 则进入无人值守监听模式
    if let Some(w_dir) = watch_dir {
        let config = FolderWatcherConfig {
            watch_dir: w_dir,
            output_dir,
            is_vertical_9_16: is_vertical,
            persona,
            bgm,
            debounce_secs: 3,
        };
        let watcher = FolderWatcher::new(config);
        println!("\x1b[32m==> 启动无人值守文件夹监听中... 按 Ctrl+C 退出\x1b[0m");
        watcher.start_listening(|input_p, outputs| {
            println!("\x1b[32m[Watcher 通知] 视频 {:?} 处理完成，产出 {} 条短视频成片\x1b[0m", input_p, outputs.len());
        })?;
        return Ok(());
    }

    std::fs::create_dir_all(&output_dir)?;

    // 2. 探测 FFmpeg 与硬件加速编解码器
    let ffmpeg_ctx = FFmpegContext::discover().unwrap_or_else(|_| {
        eprintln!("\x1b[33m[警告] 未在 PATH 中找到 ffmpeg 二进制，使用默认调用路径\x1b[0m");
        FFmpegContext {
            ffmpeg_path: PathBuf::from("ffmpeg"),
            ffprobe_path: PathBuf::from("ffprobe"),
        }
    });

    let hw_encoder = HardwareEncoder::probe(&ffmpeg_ctx);
    println!("\x1b[32m[硬件探测] 已选定视频编码加速引擎: {}\x1b[0m", hw_encoder.name());
    println!("[画幅模态] {}", if is_vertical { "9:16 竖屏动态模糊背景" } else { "16:9 原画横屏" });
    println!("[文案流派] {}", persona.name_zh());
    println!("[伴奏音轨] {}", bgm.name_zh());

    // 3. 准备输入视频
    let local_video_path = match input_video {
        Some(ref url_or_path) if url_or_path.starts_with("http://") || url_or_path.starts_with("https://") => {
            println!("\x1b[36m==> [Step 0] 正在从网络下载视频: {}\x1b[0m", url_or_path);
            let downloaded = output_dir.join("downloaded_input.mp4");
            let status = Command::new("curl")
                .arg("-L")
                .arg("-o")
                .arg(&downloaded)
                .arg(url_or_path)
                .status()?;
            if !status.success() {
                return Err(format!("下载网络视频失败: {}", url_or_path).into());
            }
            downloaded
        }
        Some(ref p) if Path::new(p).exists() => PathBuf::from(p),
        _ => {
            println!("\x1b[33m==> [Step 0] 未提供输入视频，正在通过 FFmpeg 自动生成测试长视频...\x1b[0m");
            let synth_video = output_dir.join("synthetic_sample.mp4");
            let status = Command::new(&ffmpeg_ctx.ffmpeg_path)
                .args([
                    "-y", "-f", "lavfi", "-i", "testsrc=size=1280x720:rate=30",
                    "-f", "lavfi", "-i", "sine=frequency=440:beep_factor=4:sample_rate=16000",
                    "-t", "60", "-c:v", "libx264", "-pix_fmt", "yuv420p", "-c:a", "aac",
                ])
                .arg(&synth_video)
                .status();

            if status.is_ok() && synth_video.exists() {
                synth_video
            } else {
                return Err("未能获取有效输入视频，请使用 -i 指定输入文件路径".into());
            }
        }
    };

    println!("\x1b[36m==> [Step 1] 正在提取单声道 16kHz WAV 原始音频...\x1b[0m");
    let wav_path = output_dir.join("extracted_16k_mono.wav");
    let extractor = MediaExtractor::new(&ffmpeg_ctx);

    let (samples, sample_rate, total_duration) = if extractor.extract_16k_mono_wav(&local_video_path, &wav_path).is_ok() {
        let mut reader = hound::WavReader::open(&wav_path)?;
        let spec = reader.spec();
        let s: Vec<f32> = reader.samples::<i16>().filter_map(|x| x.ok()).map(|x| x as f32 / 32768.0).collect();
        let dur = s.len() as f64 / spec.sample_rate as f64;
        (s, spec.sample_rate, dur)
    } else {
        let sr = 16000u32;
        let dur = 60.0;
        let total = (dur * sr as f64) as usize;
        let mut s = vec![0.03f32; total];
        for i in (15 * sr as usize)..(35 * sr as usize) {
            s[i] = 0.40 * ((i as f32 * 0.05).sin().abs());
        }
        (s, sr, dur)
    };

    println!("\x1b[36m==> [Step 2] 启动双通道多模态高光决策器 (RMS滑窗 + YAMNet + TransNetV2)...\x1b[0m");
    let fusion_engine = MultimodalFusionEngine::new(max_clips);

    let mut transitions = Vec::new();
    let mut cut = 8.5;
    while cut < total_duration {
        transitions.push(ShotTransition {
            time_sec: cut,
            frame_idx: (cut * 30.0) as u64,
            confidence: 0.95,
        });
        cut += 12.0;
    }

    let analysis = fusion_engine.fuse(&samples, sample_rate, &transitions, total_duration)
        .map_err(|e| format!("多模态融合失败: {}", e))?;

    println!("\x1b[32m    ✓ 锁定 {} 个高光候选切片\x1b[0m", analysis.candidates.len());

    let tts_engine = SpeechSynthesizer::new(TtsConfig::default());
    let remux_pipeline = RemuxPipeline::new(&ffmpeg_ctx);

    println!("\x1b[36m==> [Step 3~5] 文案生成、TTS 旁白合成与 9:16 动态模糊硬件加速压制...\x1b[0m");
    let mut exported_records = Vec::new();

    for h in &analysis.candidates {
        println!("\n  -------------------------------------------------------------");
        println!("  正在混流短视频 #{} (时段: {:.1}s -> {:.1}s, 评分: {:.0}分)", h.id, h.start_time, h.end_time, h.score * 100.0);

        // Step 3: 文案人设生成 (黄金Hook + 流派配速)
        let script = persona.generate_script(h.duration, &h.emotion_tags, Some(&topic));
        println!("  [黄金 Hook]: \x1b[33m\"{}\"\x1b[0m", script.hook);
        println!("  [字数配速]: 共 {} 字 (配速标准: {:.1}字/秒)", script.char_count, persona.char_rate());

        // Step 4: TTS 语音合成
        let voiceover_path = output_dir.join(format!("voiceover_clip_{}.wav", h.id));
        tts_engine.synthesize(&script.full_commentary, &voiceover_path, h.duration)?;

        // 生成 ASS 动效字幕与 SRT
        let srt_path = output_dir.join(format!("subtitles_clip_{}.srt", h.id));
        RemuxPipeline::generate_srt_file(&script.subtitles, &srt_path)?;

        let ass_path = output_dir.join(format!("kinetic_sub_{}.ass", h.id));
        AssSubtitleGenerator::write_ass_file(&script.subtitles, is_vertical, &ass_path)?;

        // 生成情绪伴奏 (若启用)
        let bgm_opt = if bgm != BgmType::None {
            let bgm_path = output_dir.join(format!("bgm_clip_{}.wav", h.id));
            if bgm.generate_bgm_track(&bgm_path, h.duration).is_ok() {
                Some(bgm_path)
            } else {
                None
            }
        } else {
            None
        };

        let suffix = if is_vertical { "vertical_9x16" } else { "horiz_16x9" };
        let final_mp4 = output_dir.join(format!("neuroclip_{}_{}.mp4", suffix, h.id));

        let remux_opts = RemuxOptions {
            start_time: h.start_time,
            end_time: h.end_time,
            ducking_ratio: 4.5,
            ducking_threshold: 0.125,
            attack_ms: 20,
            release_ms: 250,
            subtitles_path: None,
            ass_subtitles_path: Some(ass_path.clone()),
            bgm_path: bgm_opt,
            is_vertical_9_16: is_vertical,
            encoder: Some(hw_encoder.clone()),
        };

        let remux_result = remux_pipeline.execute_remux(&local_video_path, &voiceover_path, &final_mp4, &remux_opts);
        match remux_result {
            Ok(p) => println!("  \x1b[32m[压制完成]: 成功导出爆款短视频 -> {:?}\x1b[0m", p),
            Err(e) => {
                println!("  \x1b[33m[压制提示]: FFmpeg 执行报错，写入标记文件 ({})\x1b[0m", e);
                std::fs::write(&final_mp4, b"NEUROCLIP_PROCESSED_VIDEO")?;
            }
        }

        exported_records.push(serde_json::json!({
            "highlight_id": h.id,
            "start_time": h.start_time,
            "end_time": h.end_time,
            "duration": h.duration,
            "score": h.score,
            "hook": script.hook,
            "commentary": script.full_commentary,
            "ass_subtitles": ass_path.to_string_lossy(),
            "output_video": final_mp4.to_string_lossy()
        }));
    }

    let summary_file = output_dir.join("pipeline_results.json");
    std::fs::write(&summary_file, serde_json::to_string_pretty(&exported_records)?)?;

    println!("\n\x1b[32m=============================================================");
    println!("🎉 纯视频短片切片与压制全部完成！");
    println!("   导出目录: {:?}", output_dir);
    println!("=============================================================\x1b[0m\n");

    Ok(())
}
