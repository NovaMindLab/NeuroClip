use std::path::{Path, PathBuf};
use neuroclip_lib::ffmpeg::pipeline::{RemuxOptions, RemuxPipeline};
use neuroclip_lib::highlight::rms::AudioRmsAnalyzer;
use neuroclip_lib::highlight::transnet::{ShotTransition, TransNetSnapper};
use neuroclip_lib::highlight::fusion::MultimodalFusionEngine;
use neuroclip_lib::script::prompt::CommentaryGenerator;
use neuroclip_lib::tts::{SpeechSynthesizer, TtsConfig};

#[test]
fn test_rms_burst_detection() {
    let analyzer = AudioRmsAnalyzer::new(1.0, 0.5, 2.5);
    let sample_rate: u32 = 16000;
    let duration_sec = 20.0;
    let total_samples = (duration_sec * sample_rate as f64) as usize;

    let mut samples = vec![0.02f32; total_samples];
    // 在第 8 秒至 12 秒注入高能声音爆点 (振幅 0.25, 远超 2.5 倍均值)
    let burst_start = 8 * sample_rate as usize;
    let burst_end = 12 * sample_rate as usize;
    for i in burst_start..burst_end {
        samples[i] = 0.25;
    }

    let (curve, intervals) = analyzer.analyze_samples(&samples, sample_rate).expect("RMS 分析失败");
    assert!(!curve.is_empty(), "RMS 曲线不应为空");
    assert!(!intervals.is_empty(), "应成功捕获能量飙升区间");

    let first_interval = &intervals[0];
    assert!(first_interval.burst_ratio >= 2.5, "突增比率必须大于等于 2.5 倍");
    assert!(first_interval.start_sec >= 7.0 && first_interval.start_sec <= 9.0, "捕获起始时间应在爆点附近");
}

#[test]
fn test_transnet_shot_snapping() {
    let snapper = TransNetSnapper::new(3.0, 15.0, 45.0);
    let transitions = vec![
        ShotTransition { time_sec: 5.2, frame_idx: 156, confidence: 0.95 },
        ShotTransition { time_sec: 25.8, frame_idx: 774, confidence: 0.90 },
        ShotTransition { time_sec: 60.0, frame_idx: 1800, confidence: 0.92 },
    ];

    // 原始区间 [6.0, 24.5] (长度 18.5s)
    let (snapped_start, snapped_end, start_snapped, end_snapped) =
        snapper.snap_interval(6.0, 24.5, &transitions, 100.0);

    assert!(start_snapped, "起始点应成功吸附至镜头切点 5.2s");
    assert!((snapped_start - 5.2).abs() < 0.001);

    assert!(end_snapped, "结束点应成功吸附至镜头切点 25.8s");
    assert!((snapped_end - 25.8).abs() < 0.001);

    let dur = snapped_end - snapped_start;
    assert!(dur >= 15.0 && dur <= 45.0, "吸附后片段时长必须满足 15s~45s 约束");
}

#[test]
fn test_ffmpeg_remux_command_builder() {
    let dummy_video = Path::new("/path/to/raw_video.mp4");
    let dummy_voiceover = Path::new("/path/to/voiceover.wav");
    let dummy_out = Path::new("/path/to/output.mp4");
    let dummy_srt = PathBuf::from("/path/to/subtitles.srt");

    let opts = RemuxOptions {
        start_time: 10.5,
        end_time: 35.5,
        ducking_ratio: 4.0,
        ducking_threshold: 0.125,
        attack_ms: 20,
        release_ms: 250,
        subtitles_path: Some(dummy_srt),
    };

    let args = RemuxPipeline::build_remux_args(dummy_video, dummy_voiceover, dummy_out, &opts);
    let args_str = args.join(" ");

    // 验证核心命令滤镜
    assert!(args_str.contains("sidechaincompress=threshold=0.125:ratio=4:attack=20:release=250"), "必须包含 sidechaincompress 闪避滤镜");
    assert!(args_str.contains("amix=inputs=2:duration=first:dropout_transition=2"), "必须包含 amix 双路混音");
    assert!(args_str.contains("subtitles="), "必须包含硬字幕滤镜烧录");
    assert!(args_str.contains("-c:v libx264"), "视频编码必须为 libx264");
    assert!(args_str.contains("-c:a aac"), "音频编码必须为 aac");
    assert!(args_str.contains("-ss 10.500"), "精准开始时间");
    assert!(args_str.contains("-t 25.000"), "精准切片时长 (35.5 - 10.5 = 25.0s)");
}

#[test]
fn test_commentary_hook_and_pacing() {
    let duration_sec: f64 = 20.0;
    let expected_target_chars = (duration_sec * 4.2f64).round() as usize; // 84 字
    assert_eq!(CommentaryGenerator::target_char_count(duration_sec), expected_target_chars);

    let script = CommentaryGenerator::generate_heuristic_script(
        duration_sec,
        &["欢呼声".to_string()],
        Some("逆风翻盘"),
    );

    // 验证前 3 秒黄金 Hook
    assert!(script.hook.starts_with("注意看") || script.hook.starts_with("谁能想到") || script.hook.starts_with("千万别眨眼"));
    assert_eq!(script.subtitles[0].start_sec, 0.0);
    assert_eq!(script.subtitles[0].end_sec, 3.0);

    // 验证字数误差在 ±4 字以内
    let diff = (script.char_count as isize - expected_target_chars as isize).abs();
    assert!(diff <= 4, "字数应严密逼近 4.2字/秒（实际: {}, 目标: {}）", script.char_count, expected_target_chars);

    // 验证生成 SRT
    let temp_srt = std::env::temp_dir().join("neuroclip_test_sub.srt");
    RemuxPipeline::generate_srt_file(&script.subtitles, &temp_srt).expect("生成 SRT 失败");
    let content = std::fs::read_to_string(&temp_srt).unwrap();
    assert!(content.contains("00:00:00,000 --> 00:00:03,000"));
    let _ = std::fs::remove_file(temp_srt);
}

#[test]
fn test_multimodal_fusion_flow() {
    let engine = MultimodalFusionEngine::new(3);
    let sample_rate: u32 = 16000;
    let duration = 60.0;
    let total_samples = (duration * sample_rate as f64) as usize;
    let mut samples = vec![0.02f32; total_samples];

    // 在 20~30s 注入爆点
    for i in (20 * sample_rate as usize)..(30 * sample_rate as usize) {
        samples[i] = 0.30;
    }

    let transitions = vec![
        ShotTransition { time_sec: 18.0, frame_idx: 540, confidence: 0.9 },
        ShotTransition { time_sec: 38.0, frame_idx: 1140, confidence: 0.9 },
    ];

    let result = engine.fuse(&samples, sample_rate, &transitions, duration).expect("多模态融合失败");
    assert!(!result.candidates.is_empty(), "应识别出高光片段");
    let top_clip = &result.candidates[0];
    assert!(top_clip.duration >= 15.0 && top_clip.duration <= 45.0, "高光片段时长必须在 15s~45s 之间");
}

#[test]
fn test_tts_wav_synthesis() {
    let tts = SpeechSynthesizer::new(TtsConfig::default());
    let temp_wav = std::env::temp_dir().join("neuroclip_test_tts.wav");
    let text = "注意看！这波操作堪称封神瞬间！";
    let target_dur = 3.5;

    let res = tts.synthesize(text, &temp_wav, target_dur);
    assert!(res.is_ok(), "TTS 离线合成必须成功");
    assert!(temp_wav.exists(), "生成的 WAV 文件必须存在");

    let reader = hound::WavReader::open(&temp_wav).expect("校验 WAV 格式失败");
    let spec = reader.spec();
    assert_eq!(spec.channels, 1, "必须为单声道");
    assert_eq!(spec.sample_rate, 16000, "采样率必须为 16000");

    let _ = std::fs::remove_file(temp_wav);
}
