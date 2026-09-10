use std::path::{Path, PathBuf};
use neuroclip_lib::ffmpeg::pipeline::{RemuxOptions, RemuxPipeline, SubtitleCue};
use neuroclip_lib::ffmpeg::ass::AssSubtitleGenerator;
use neuroclip_lib::ffmpeg::hwaccel::HardwareEncoder;
use neuroclip_lib::highlight::rms::AudioRmsAnalyzer;
use neuroclip_lib::highlight::transnet::{ShotTransition, TransNetSnapper};
use neuroclip_lib::script::persona::CommentaryPersona;
use neuroclip_lib::tts::BgmType;

#[test]
fn test_rms_burst_detection() {
    let analyzer = AudioRmsAnalyzer::new(1.0, 0.5, 2.5);
    let sample_rate: u32 = 16000;
    let duration_sec = 20.0;
    let total_samples = (duration_sec * sample_rate as f64) as usize;

    let mut samples = vec![0.02f32; total_samples];
    // 在第 8 秒至 12 秒注入高能声音爆点
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
        ass_subtitles_path: None,
        bgm_path: None,
        is_vertical_9_16: false,
        encoder: Some(HardwareEncoder::CpuLibx264),
    };

    let args = RemuxPipeline::build_remux_args(dummy_video, dummy_voiceover, dummy_out, &opts);
    let args_str = args.join(" ");

    assert!(args_str.contains("sidechaincompress=threshold=0.125:ratio=4:attack=20:release=250"), "必须包含 sidechaincompress 闪避滤镜");
    assert!(args_str.contains("amix=inputs=2:duration=first:dropout_transition=2"), "必须包含 amix 双路混音");
    assert!(args_str.contains("subtitles="), "必须包含硬字幕滤镜烧录");
    assert!(args_str.contains("-c:v libx264"), "视频编码必须包含 libx264");
    assert!(args_str.contains("-c:a aac"), "音频编码必须为 aac");
}

#[test]
fn test_ass_kinetic_subtitles_generation() {
    let cues = vec![
        SubtitleCue { start_sec: 0.0, end_sec: 3.0, text: "注意看！就在这一秒".to_string() },
        SubtitleCue { start_sec: 3.0, end_sec: 6.5, text: "极限反杀瞬间引爆全场".to_string() },
    ];

    let script = AssSubtitleGenerator::generate_ass_script(&cues, true);
    assert!(script.contains("PlayResX: 1080"), "竖屏分辨率 X 应为 1080");
    assert!(script.contains("PlayResY: 1920"), "竖屏分辨率 Y 应为 1920");
    assert!(script.contains("Style: Highlight"), "应包含高光强调样式");
    assert!(script.contains("0:00:00.00"), "起始时间戳格式必须符合 ASS 规范");
    assert!(script.contains("0:00:03.00"), "结束时间戳格式必须符合 ASS 规范");
}

#[test]
fn test_vertical_9_16_blur_filter() {
    let dummy_video = Path::new("/path/to/raw_video.mp4");
    let dummy_voiceover = Path::new("/path/to/voiceover.wav");
    let dummy_out = Path::new("/path/to/output.mp4");
    let dummy_ass = PathBuf::from("/path/to/subtitles.ass");

    let opts = RemuxOptions {
        start_time: 0.0,
        end_time: 25.0,
        ducking_ratio: 4.5,
        ducking_threshold: 0.125,
        attack_ms: 20,
        release_ms: 250,
        subtitles_path: None,
        ass_subtitles_path: Some(dummy_ass),
        bgm_path: None,
        is_vertical_9_16: true,
        encoder: Some(HardwareEncoder::VideoToolbox),
    };

    let args = RemuxPipeline::build_remux_args(dummy_video, dummy_voiceover, dummy_out, &opts);
    let args_str = args.join(" ");

    assert!(args_str.contains("boxblur=25:5"), "9:16 竖屏模式必须包含高斯模糊背景");
    assert!(args_str.contains("scale=1080:1920"), "必须缩放至 1080x1920");
    assert!(args_str.contains("ass="), "必须包含 ass 动效字幕滤镜");
    assert!(args_str.contains("h264_videotoolbox"), "硬件加速参数应包含 h264_videotoolbox");
}

#[test]
fn test_three_track_bgm_ducking() {
    let dummy_video = Path::new("/path/to/raw_video.mp4");
    let dummy_voiceover = Path::new("/path/to/voiceover.wav");
    let dummy_bgm = PathBuf::from("/path/to/bgm.wav");
    let dummy_out = Path::new("/path/to/output.mp4");

    let opts = RemuxOptions {
        start_time: 0.0,
        end_time: 30.0,
        ducking_ratio: 4.5,
        ducking_threshold: 0.125,
        attack_ms: 20,
        release_ms: 250,
        subtitles_path: None,
        ass_subtitles_path: None,
        bgm_path: Some(dummy_bgm),
        is_vertical_9_16: false,
        encoder: None,
    };

    let args = RemuxPipeline::build_remux_args(dummy_video, dummy_voiceover, dummy_out, &opts);
    let args_str = args.join(" ");

    assert!(args_str.contains("amix=inputs=3"), "挂载 BGM 时必须使用 3 轨混音");
    assert!(args_str.contains("[2:a][1:a]sidechaincompress"), "必须对 BGM 伴奏施加侧链闪避");
}

#[test]
fn test_multi_persona_script_pacing() {
    let duration_sec = 20.0;

    let personas = [
        CommentaryPersona::SuspenseDrama,
        CommentaryPersona::HighEnergyEsports,
        CommentaryPersona::HumorousRoast,
        CommentaryPersona::TacticalBreakdown,
    ];

    for p in personas {
        let script = p.generate_script(duration_sec, &["欢呼声".to_string()], Some("逆风翻盘"));
        assert!(script.char_count > 50, "{} 文案字数不应为空", p.name_zh());
        assert!(!script.hook.is_empty(), "{} 必须生成黄金 Hook", p.name_zh());
        assert!(!script.subtitles.is_empty(), "{} 必须切分字幕时间轴", p.name_zh());
    }
}

#[test]
fn test_bgm_generation() {
    let temp_bgm = std::env::temp_dir().join("neuroclip_test_bgm.wav");
    let bgm = BgmType::HypeTrap;
    let res = bgm.generate_bgm_track(&temp_bgm, 5.0);

    assert!(res.is_ok(), "BGM 生成必须成功");
    assert!(temp_bgm.exists(), "生成的 BGM 文件必须存在");

    let reader = hound::WavReader::open(&temp_bgm).unwrap();
    let spec = reader.spec();
    assert_eq!(spec.sample_rate, 16000);
    let _ = std::fs::remove_file(temp_bgm);
}
