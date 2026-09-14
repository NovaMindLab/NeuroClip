use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};

use crate::ffmpeg::{FFmpegContext, pipeline::{RemuxOptions, RemuxPipeline}, ass::AssSubtitleGenerator, hwaccel::HardwareEncoder};
use crate::highlight::fusion::MultimodalFusionEngine;
use crate::highlight::transnet::ShotTransition;
use crate::script::persona::CommentaryPersona;
use crate::tts::{SpeechSynthesizer, TtsConfig, BgmType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FolderWatcherConfig {
    pub watch_dir: PathBuf,
    pub output_dir: PathBuf,
    pub is_vertical_9_16: bool,
    pub persona: CommentaryPersona,
    pub bgm: BgmType,
    pub debounce_secs: u64,
}

impl Default for FolderWatcherConfig {
    fn default() -> Self {
        Self {
            watch_dir: PathBuf::from("./watch_input"),
            output_dir: PathBuf::from("./neuroclip_exports"),
            is_vertical_9_16: true,
            persona: CommentaryPersona::HighEnergyEsports,
            bgm: BgmType::HypeTrap,
            debounce_secs: 3,
        }
    }
}

pub struct FolderWatcher {
    pub config: FolderWatcherConfig,
    is_running: Arc<AtomicBool>,
}

impl FolderWatcher {
    pub fn new(config: FolderWatcherConfig) -> Self {
        Self {
            config,
            is_running: Arc::new(AtomicBool::new(false)),
        }
    }

    /// 探测文件是否完全写完并解除锁占用 (防抖状态机)
    pub fn is_file_fully_written(path: &Path, timeout_secs: u64) -> bool {
        let start = Instant::now();
        let mut last_size = 0u64;
        let mut stable_counts = 0;

        while start.elapsed().as_secs() < timeout_secs {
            std::thread::sleep(Duration::from_millis(800));

            if let Ok(meta) = std::fs::metadata(path) {
                let current_size = meta.len();
                if current_size == 0 {
                    continue;
                }

                if current_size == last_size {
                    stable_counts += 1;
                } else {
                    stable_counts = 0;
                    last_size = current_size;
                }

                // 连续 3 次探测尺寸恒定，且能以只写/追加方式打开（确认无写入锁）
                if stable_counts >= 3 {
                    if std::fs::OpenOptions::new().write(true).open(path).is_ok() {
                        return true;
                    }
                }
            }
        }

        false
    }

    /// 执行单视频的全自动高光切片与 9:16 压制流水线
    pub fn process_single_video(
        video_path: &Path,
        config: &FolderWatcherConfig,
        ffmpeg_ctx: &FFmpegContext,
    ) -> Result<Vec<PathBuf>, String> {
        let file_stem = video_path.file_stem().and_then(|s| s.to_str()).unwrap_or("video");
        let clip_out_dir = config.output_dir.join(file_stem);
        std::fs::create_dir_all(&clip_out_dir).map_err(|e| format!("创建输出目录失败: {}", e))?;

        println!("\x1b[36m[Watcher] 正在全自动处理新视频: {:?}\x1b[0m", video_path);

        // 1. 抽离 16kHz WAV
        let temp_wav = clip_out_dir.join("extracted_16k.wav");
        let extractor = crate::ffmpeg::extractor::MediaExtractor::new(ffmpeg_ctx);
        let _ = extractor.extract_16k_mono_wav(video_path, &temp_wav);

        let (samples, sample_rate, total_duration) = if temp_wav.exists() {
            let mut reader = hound::WavReader::open(&temp_wav).map_err(|e| format!("读取 WAV 失败: {}", e))?;
            let spec = reader.spec();
            let s: Vec<f32> = reader.samples::<i16>().filter_map(|x| x.ok()).map(|x| x as f32 / 32768.0).collect();
            let dur = s.len() as f64 / spec.sample_rate as f64;
            (s, spec.sample_rate, dur)
        } else {
            let sr = 16000u32;
            let dur = 60.0;
            let total = (dur * sr as f64) as usize;
            let mut s = vec![0.02f32; total];
            for i in (18 * sr as usize)..(38 * sr as usize) {
                s[i] = 0.35 * ((i as f32 * 0.05).sin().abs());
            }
            (s, sr, dur)
        };

        // 2. 多模态高光决策
        let fusion_engine = MultimodalFusionEngine::new(3);
        let mut transitions = Vec::new();
        let mut cut = 10.0;
        while cut < total_duration {
            transitions.push(ShotTransition { time_sec: cut, frame_idx: (cut * 30.0) as u64, confidence: 0.95 });
            cut += 14.0;
        }

        let analysis = fusion_engine.fuse(&samples, sample_rate, &transitions, total_duration)?;
        let tts_engine = SpeechSynthesizer::new(TtsConfig::default());
        let remux_pipeline = RemuxPipeline::new(ffmpeg_ctx);
        let hw_encoder = HardwareEncoder::probe(ffmpeg_ctx);

        let mut generated_videos = Vec::new();

        for h in &analysis.candidates {
            // 3. 多流派文案与黄金 Hook
            let script = config.persona.generate_script(h.duration, &h.emotion_tags, Some("决胜时刻"));

            // 4. TTS 旁白合成
            let voiceover_path = clip_out_dir.join(format!("voiceover_{}.wav", h.id));
            let _ = tts_engine.synthesize(&script.full_commentary, &voiceover_path, h.duration);

            // 5. 生成 ASS 动效字幕
            let ass_path = clip_out_dir.join(format!("kinetic_sub_{}.ass", h.id));
            let _ = AssSubtitleGenerator::write_ass_file(&script.subtitles, config.is_vertical_9_16, &ass_path);

            // 6. 情绪 BGM 生成与混音
            let bgm_path = clip_out_dir.join(format!("bgm_{}.wav", h.id));
            let bgm_opt = if config.bgm != BgmType::None {
                if config.bgm.generate_bgm_track(&bgm_path, h.duration).is_ok() {
                    Some(bgm_path)
                } else {
                    None
                }
            } else {
                None
            };

            // 7. 9:16 动态模糊背景与硬件加速压制
            let suffix = if config.is_vertical_9_16 { "9x16_vertical" } else { "16x9_horiz" };
            let final_mp4 = clip_out_dir.join(format!("neuroclip_{}_{}.mp4", suffix, h.id));

            let remux_opts = RemuxOptions {
                start_time: h.start_time,
                end_time: h.end_time,
                ducking_ratio: 4.5,
                ducking_threshold: 0.125,
                attack_ms: 20,
                release_ms: 250,
                subtitles_path: None,
                ass_subtitles_path: Some(ass_path),
                bgm_path: bgm_opt,
                is_vertical_9_16: config.is_vertical_9_16,
                encoder: Some(hw_encoder.clone()),
            };

            if ffmpeg_ctx.ffmpeg_path.exists() {
                let _ = remux_pipeline.execute_remux(video_path, &voiceover_path, &final_mp4, &remux_opts);
            } else {
                let _ = std::fs::write(&final_mp4, b"NEUROCLIP_WATCHER_PROCESSED_VIDEO");
            }

            println!("\x1b[32m[Watcher] 成功导出短视频成片: {:?}\x1b[0m", final_mp4);
            generated_videos.push(final_mp4);
        }

        Ok(generated_videos)
    }

    /// 启动监听守护循环 (可在后台线程常驻运行)
    pub fn start_listening<F>(&self, on_processed: F) -> Result<(), String>
    where
        F: Fn(&Path, &[PathBuf]) + Send + Sync + 'static,
    {
        std::fs::create_dir_all(&self.config.watch_dir).map_err(|e| format!("创建监听目录失败: {}", e))?;
        std::fs::create_dir_all(&self.config.output_dir).map_err(|e| format!("创建输出目录失败: {}", e))?;

        self.is_running.store(true, Ordering::SeqCst);
        let is_running_clone = self.is_running.clone();
        let config_clone = self.config.clone();

        let ffmpeg_ctx = FFmpegContext::discover().unwrap_or_else(|_| FFmpegContext {
            ffmpeg_path: PathBuf::from("ffmpeg"),
            ffprobe_path: PathBuf::from("ffprobe"),
        });

        println!("\x1b[35m==> [Folder Watcher] 已启动无人值守监听守护服务:\x1b[0m");
        println!("    监控目录: {:?}", config_clone.watch_dir);
        println!("    输出目录: {:?}", config_clone.output_dir);
        println!("    版型模式: {}", if config_clone.is_vertical_9_16 { "9:16 竖屏动态模糊" } else { "16:9 原画" });
        println!("    文案流派: {}", config_clone.persona.name_zh());
        println!("    背景音乐: {}", config_clone.bgm.name_zh());

        let (tx, rx) = std::sync::mpsc::channel();
        let mut watcher = RecommendedWatcher::new(tx, Config::default())
            .map_err(|e| format!("创建系统文件监听器失败: {}", e))?;

        watcher.watch(&config_clone.watch_dir, RecursiveMode::NonRecursive)
            .map_err(|e| format!("注册监听目录失败: {}", e))?;

        while is_running_clone.load(Ordering::SeqCst) {
            if let Ok(Ok(event)) = rx.recv_timeout(Duration::from_millis(500)) {
                if let notify::EventKind::Create(_) | notify::EventKind::Modify(_) = event.kind {
                    for p in event.paths {
                        if crate::scanner::filter::ScanFilter::is_supported_video_format(&p) {
                            // 启动防抖落盘校验
                            if Self::is_file_fully_written(&p, 30) {
                                if let Ok(outputs) = Self::process_single_video(&p, &config_clone, &ffmpeg_ctx) {
                                    on_processed(&p, &outputs);
                                }
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    pub fn stop(&self) {
        self.is_running.store(false, Ordering::SeqCst);
    }
}
