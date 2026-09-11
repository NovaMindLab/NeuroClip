import React, { useState, useEffect, Suspense, lazy } from "react";
import { AppLayout } from "./components/layout/AppLayout";
import { VideoDropzone } from "./components/VideoDropzone";
import { PipelineStepper } from "./components/PipelineStepper";
import { TimelineWaveform } from "./components/TimelineWaveform";
import { HighlightList, HighlightItem } from "./components/HighlightList";
import { CommentaryStudio, CommentaryData } from "./components/CommentaryStudio";
import { ClipPlayerModal } from "./components/ClipPlayerModal";
import { ArchitectureModal } from "./components/ArchitectureModal";
import { AppNavTab, MediaAssetItem } from "./types/library";
import { Eye, FolderGit2, HardDrive, Sparkles } from "lucide-react";

// 次屏重量级视图动态代码切片懒加载，大幅提升首屏加载速度
const MediaLibraryView = lazy(() =>
  import("./components/media-library/MediaLibraryView").then((m) => ({ default: m.MediaLibraryView }))
);
const SettingsView = lazy(() =>
  import("./components/settings/SettingsView").then((m) => ({ default: m.SettingsView }))
);

function generateInitialRmsCurve(): [number, number][] {
  const points: [number, number][] = [];
  for (let t = 0; t <= 180; t += 1.2) {
    let r = 0.05 + Math.sin(t * 0.2) * 0.02;
    if (t >= 16 && t <= 38) r += 0.35 * Math.abs(Math.sin(t * 0.8));
    if (t >= 70 && t <= 96) r += 0.28 * Math.abs(Math.cos(t * 0.6));
    if (t >= 124 && t <= 144) r += 0.24 * Math.abs(Math.sin(t * 0.9));
    points.push([t, r]);
  }
  return points;
}

export const App: React.FC = () => {
  // Navigation layout state
  const [activeTab, setActiveTab] = useState<AppNavTab>("studio");
  const [libraryCount, setLibraryCount] = useState<number>(3);
  const [toastMessage, setToastMessage] = useState<string | null>(null);

  const [selectedVideoPath, setSelectedVideoPath] = useState<string | null>(
    "/samples/esports_final_championship_match.mp4"
  );
  const [selectedVideoName, setSelectedVideoName] = useState<string | null>(
    "esports_final_championship_match.mp4"
  );

  const [isProcessing, setIsProcessing] = useState(false);
  const [currentStep, setCurrentStep] = useState<string>("idle");
  const [progress, setProgress] = useState<number>(0);
  const [statusMessage, setStatusMessage] = useState<string>("");

  const [isInfoModalOpen, setIsInfoModalOpen] = useState(false);
  const [isPlayerModalOpen, setIsPlayerModalOpen] = useState(false);

  // States for Phase 2 & 3: 9:16 Aspect ratio, Personas, BGM, and Folder Watcher
  const [isVertical, setIsVertical] = useState(true);
  const [currentPersona, setCurrentPersona] = useState("esports");
  const [currentBgm, setCurrentBgm] = useState("trap");
  const [hwEncoderName, setHwEncoderName] = useState("VideoToolbox (Apple Silicon 硬件加速)");
  const [isWatcherActive, setIsWatcherActive] = useState(false);

  // Initial highlight candidates
  const [highlights, setHighlights] = useState<HighlightItem[]>([
    {
      id: 1,
      start_time: 14.5,
      end_time: 39.5,
      duration: 25.0,
      score: 0.96,
      burst_ratio: 4.2,
      emotion_tags: ["欢呼声", "热烈掌声"],
      start_snapped: true,
      end_snapped: true,
      summary: "高光 #1: 能量飙升 4.2x，检测到[欢呼声, 热烈掌声]，时长 25.0秒",
    },
    {
      id: 2,
      start_time: 68.0,
      end_time: 98.0,
      duration: 30.0,
      score: 0.89,
      burst_ratio: 3.5,
      emotion_tags: ["尖叫/呐喊"],
      start_snapped: true,
      end_snapped: true,
      summary: "高光 #2: 能量飙升 3.5x，检测到[尖叫/呐喊]，时长 30.0秒",
    },
    {
      id: 3,
      start_time: 122.0,
      end_time: 146.0,
      duration: 24.0,
      score: 0.84,
      burst_ratio: 3.1,
      emotion_tags: ["大笑", "欢呼声"],
      start_snapped: true,
      end_snapped: false,
      summary: "高光 #3: 能量飙升 3.1x，检测到[大笑, 欢呼声]，时长 24.0秒",
    },
  ]);

  const [selectedHighlightId, setSelectedHighlightId] = useState<number>(1);
  const [rmsCurve] = useState<[number, number][]>(generateInitialRmsCurve);

  useEffect(() => {
    // 毫秒级探测硬件加速与系统状态 (Fast-Path 内存读取)
    (async () => {
      try {
        const { invoke } = await import("@tauri-apps/api/core");
        const status = await invoke<any>("get_system_status");
        if (status?.hw_encoder) {
          setHwEncoderName(status.hw_encoder);
        }
      } catch {
        // Fallback in web preview
      }
    })();
  }, []);

  const selectedHighlight = highlights.find((h) => h.id === selectedHighlightId) || highlights[0];

  const [commentaryMap, setCommentaryMap] = useState<Record<number, CommentaryData>>({
    1: {
      hook: "千万别眨眼！这波决胜神级操作直接把全场看傻了！",
      full_commentary:
        "千万别眨眼！这波决胜神级操作直接把全场看傻了！行云流水的极限走位，每一个技能都精准卡在毫厘之间，伤害瞬间拉满，全场观众集体起立陷入狂欢，这波操作直接封神！现场瞬间彻底沸腾！",
      duration_seconds: 25.0,
      char_count: 125,
      subtitles: [
        { start_sec: 0.0, end_sec: 3.0, text: "千万别眨眼！这波决胜神级操作直接把全场看傻了" },
        { start_sec: 3.0, end_sec: 8.5, text: "行云流水的极限走位" },
        { start_sec: 8.5, end_sec: 14.2, text: "每一个技能都精准卡在毫厘之间" },
        { start_sec: 14.2, end_sec: 19.8, text: "伤害瞬间拉满，全场观众集体起立陷入狂欢" },
        { start_sec: 19.8, end_sec: 25.0, text: "这波操作直接封神！现场彻底沸腾" },
      ],
    },
    2: {
      hook: "注意看！谁能想到原本死局的对线，竟埋下了惊天伏笔！",
      full_commentary:
        "注意看！谁能想到原本死局的对线，竟埋下了惊天伏笔！就在所有人都以为局势已定时，关键细节悄然逆转，呼吸之间胜负彻底颠覆，让人不得不倒吸一口凉气！这绝对是不可多得的名场面！",
      duration_seconds: 30.0,
      char_count: 120,
      subtitles: [
        { start_sec: 0.0, end_sec: 3.0, text: "注意看！谁能想到原本死局的对线，竟埋下了惊天伏笔" },
        { start_sec: 3.0, end_sec: 11.2, text: "就在所有人都以为局势已定时" },
        { start_sec: 11.2, end_sec: 18.5, text: "关键细节悄然逆转" },
        { start_sec: 18.5, end_sec: 24.2, text: "呼吸之间胜负彻底颠覆，让人不得不倒吸一口凉气" },
        { start_sec: 24.2, end_sec: 30.0, text: "这绝对是不可多得的名场面" },
      ],
    },
    3: {
      hook: "原谅我不厚道地笑了！这波下饭操作直接承包整晚笑点！",
      full_commentary:
        "原谅我不厚道地笑了！这波下饭操作直接承包整晚笑点！本以为是个王者降临，没想到反手就是一个意料之外的神级下饭名场面，现场解说都差点没绷住，简直太魔性了！这一刻注定成为名场面！",
      duration_seconds: 24.0,
      char_count: 96,
      subtitles: [
        { start_sec: 0.0, end_sec: 3.0, text: "原谅我不厚道地笑了！这波操作直接承包整晚笑点" },
        { start_sec: 3.0, end_sec: 10.5, text: "本以为是个王者降临，没想到反手就是一个神级下饭名场面" },
        { start_sec: 10.5, end_sec: 17.2, text: "现场解说都差点没绷住，简直太魔性了" },
        { start_sec: 17.2, end_sec: 24.0, text: "不得不说，这一刻注定成为经典名场面" },
      ],
    },
  });

  const currentCommentary = commentaryMap[selectedHighlightId] || commentaryMap[1];

  const handleStartProcess = async () => {
    if (!selectedVideoPath) return;

    setIsProcessing(true);
    setProgress(0.05);
    setCurrentStep("extracting_audio");
    setStatusMessage("Step 1: FFmpeg 正在抽离单声道 16kHz WAV 音轨...");

    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const { listen } = await import("@tauri-apps/api/event");

      const unlisten = await listen<any>("clip://progress", (event) => {
        setCurrentStep(event.payload.step);
        setProgress(event.payload.progress);
        setStatusMessage(event.payload.message);
      });

      const res = await invoke<any>("start_auto_clip", {
        videoPath: selectedVideoPath,
        isVertical916: isVertical,
        persona: currentPersona,
        bgm: currentBgm,
      });

      if (res?.highlights) {
        setHighlights(res.highlights);
      }

      unlisten();
    } catch {
      // Browser / Dev mock simulation runner
      const steps = [
        { step: "extracting_audio", p: 0.18, msg: "Step 1: 抽离 16kHz 单声道 WAV + 抽帧采样完成" },
        { step: "analyzing_highlights", p: 0.42, msg: "Step 2: RMS >2.5x 爆点挖掘 + YAMNet 情绪识别 + TransNetV2 镜头吸附完成" },
        { step: "generating_commentary", p: 0.62, msg: `Step 3: 调用${currentPersona}流派：黄金Hook锁定，字数严格对齐完成` },
        { step: "synthesizing_tts", p: 0.80, msg: "Step 4: 离线合成 16kHz WAV 旁白 + 生成 ASS 逐字跳动动效字幕完成" },
        { step: "rendering_video", p: 1.0, msg: `Step 5: ${isVertical ? "9:16 竖屏动态模糊" : "16:9 原画"} + BGM 混音卡点 + 硬件加速压制完成！` },
      ];

      for (const s of steps) {
        await new Promise((r) => setTimeout(r, 650));
        setCurrentStep(s.step);
        setProgress(s.p);
        setStatusMessage(s.msg);
      }
      setHighlights((prev) => [...prev]);
    } finally {
      setIsProcessing(false);
    }
  };

  const handleToggleWatcher = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const active = await invoke<boolean>("toggle_folder_watcher", {
        watchDir: "./watch_input",
        outputDir: "./neuroclip_exports",
        isVertical,
      });
      setIsWatcherActive(active);
    } catch {
      setIsWatcherActive(!isWatcherActive);
    }
  };

  const handleUpdateCommentaryText = (newText: string) => {
    setCommentaryMap((prev) => ({
      ...prev,
      [selectedHighlightId]: {
        ...prev[selectedHighlightId],
        full_commentary: newText,
        char_count: newText.length,
      },
    }));
  };

  // 核心跨模块联动：从媒体库载入工作台并切换视图
  const handleLoadAssetIntoStudio = (asset: MediaAssetItem) => {
    setSelectedVideoPath(asset.fullPath);
    setSelectedVideoName(asset.name);
    setActiveTab("studio");
    setToastMessage(`已将《${asset.name}》载入高光工作台，准备开始自动化切片！`);

    setTimeout(() => {
      setToastMessage(null);
    }, 4000);
  };

  return (
    <AppLayout
      activeTab={activeTab}
      onSelectTab={setActiveTab}
      hwEncoderName={hwEncoderName}
      isWatcherActive={isWatcherActive}
      onToggleWatcher={handleToggleWatcher}
      ffmpegReady={true}
      onOpenInfoModal={() => setIsInfoModalOpen(true)}
      libraryCount={libraryCount}
    >
      {/* 全局 Toast 通知 */}
      {toastMessage && (
        <div className="fixed top-20 right-8 z-50 flex items-center gap-2.5 px-4 py-2.5 rounded-xl bg-cyan-950/90 border border-cyan-400 text-cyan-200 text-xs font-semibold shadow-2xl shadow-cyan-500/30 backdrop-blur-md animate-bounce">
          <Sparkles className="w-4 h-4 text-amber-300" />
          <span>{toastMessage}</span>
        </div>
      )}

      {/* 视图 1：高光剪辑工作台 (Studio) */}
      {activeTab === "studio" && (
        <div className="space-y-5 max-w-[1600px] mx-auto pb-10">
          {/* Top Dropzone with 9:16 vs 16:9 Switcher */}
          <VideoDropzone
            selectedVideoPath={selectedVideoPath}
            selectedVideoName={selectedVideoName}
            onVideoSelected={(p, n) => {
              setSelectedVideoPath(p);
              setSelectedVideoName(n);
            }}
            isProcessing={isProcessing}
            onStartProcess={handleStartProcess}
            isVertical={isVertical}
            onToggleAspect={(v) => setIsVertical(v)}
          />

          {/* 5-Step Pipeline Stepper */}
          <PipelineStepper
            currentStep={currentStep}
            progress={progress}
            statusMessage={statusMessage}
          />

          {/* Multimodal Timeline & Waveform */}
          <TimelineWaveform
            totalDuration={180.0}
            rmsCurve={rmsCurve}
            selectedStart={selectedHighlight.start_time}
            selectedEnd={selectedHighlight.end_time}
          />

          {/* Dual-Column Interactive Studio */}
          <div className="grid grid-cols-1 lg:grid-cols-12 gap-5">
            <div className="lg:col-span-5">
              <HighlightList
                highlights={highlights}
                selectedId={selectedHighlightId}
                onSelectHighlight={(id) => setSelectedHighlightId(id)}
              />
            </div>

            <div className="lg:col-span-7">
              <CommentaryStudio
                data={currentCommentary}
                onUpdateCommentary={handleUpdateCommentaryText}
                onPreviewTts={() => {}}
                onRegenerate={() => {}}
                onOpenPlayer={() => setIsPlayerModalOpen(true)}
                currentPersona={currentPersona}
                onChangePersona={(p) => setCurrentPersona(p)}
                currentBgm={currentBgm}
                onChangeBgm={(b) => setCurrentBgm(b)}
              />
            </div>
          </div>
        </div>
      )}

      {/* 视图 2：PC 本地视频资产库 (Media Library) */}
      {activeTab === "library" && (
        <Suspense fallback={<div className="p-8 text-center text-slate-500 font-mono text-xs">正在载入媒体库引擎...</div>}>
          <MediaLibraryView
            onLoadIntoStudio={handleLoadAssetIntoStudio}
            onUpdateCount={setLibraryCount}
          />
        </Suspense>
      )}

      {/* 视图 3：无人值守监听中台 (Watcher) */}
      {activeTab === "watcher" && (
        <div className="space-y-6 max-w-[1200px] mx-auto pb-10">
          <div className="p-6 rounded-2xl bg-[#121622] border border-[#1e2433] shadow-xl space-y-4">
            <div className="flex items-center justify-between">
              <div className="flex items-center gap-3">
                <div className={`w-10 h-10 rounded-xl flex items-center justify-center ${
                  isWatcherActive ? "bg-emerald-950/80 border border-emerald-500/40 text-emerald-400" : "bg-slate-900 border border-slate-700 text-slate-400"
                }`}>
                  <Eye className="w-5 h-5" />
                </div>
                <div>
                  <h2 className="text-sm font-bold text-white">无人值守守护进程控制台</h2>
                  <p className="text-xs text-slate-400">基于 Rust notify 跨平台监听，连续3次稳定探测防写入锁冲突</p>
                </div>
              </div>

              <button
                onClick={handleToggleWatcher}
                className={`px-4 py-2 rounded-xl text-xs font-bold transition-all shadow-lg ${
                  isWatcherActive
                    ? "bg-emerald-600 hover:bg-emerald-500 text-white shadow-emerald-500/20"
                    : "bg-slate-800 hover:bg-slate-700 text-slate-200 border border-slate-700"
                }`}
              >
                {isWatcherActive ? "暂停监听服务" : "启动无人值守监听"}
              </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4 pt-4 border-t border-slate-800">
              <div className="p-4 rounded-xl bg-[#0a0c10] border border-slate-800">
                <div className="text-xs text-slate-400 mb-1 flex items-center gap-1.5">
                  <FolderGit2 className="w-3.5 h-3.5 text-cyan-400" />
                  <span>监控输入目录 (Watch Input)</span>
                </div>
                <div className="font-mono text-xs text-slate-200">./watch_input</div>
              </div>

              <div className="p-4 rounded-xl bg-[#0a0c10] border border-slate-800">
                <div className="text-xs text-slate-400 mb-1 flex items-center gap-1.5">
                  <HardDrive className="w-3.5 h-3.5 text-purple-400" />
                  <span>自动化成品导出目录 (Exports)</span>
                </div>
                <div className="font-mono text-xs text-slate-200">./neuroclip_exports</div>
              </div>
            </div>
          </div>
        </div>
      )}

      {/* 视图 4：系统配置与在线升级中心 (Settings) */}
      {activeTab === "settings" && (
        <Suspense fallback={<div className="p-8 text-center text-slate-500 font-mono text-xs">正在载入系统配置与更新向导...</div>}>
          <SettingsView hwEncoderName={hwEncoderName} />
        </Suspense>
      )}

      {/* Preview Modal supporting 9:16 smartphone mockup frame */}
      <ClipPlayerModal
        isOpen={isPlayerModalOpen}
        onClose={() => setIsPlayerModalOpen(false)}
        clipTitle={`高光切片 #${selectedHighlight.id} - ${selectedHighlight.emotion_tags.join(" + ")}`}
        duration={selectedHighlight.duration}
        commentary={currentCommentary}
        videoPath={`/neuroclip_exports/neuroclip_${isVertical ? "9x16_vertical" : "16x9"}_${selectedHighlight.id}.mp4`}
        isVertical={isVertical}
      />

      {/* Architecture Modal */}
      <ArchitectureModal
        isOpen={isInfoModalOpen}
        onClose={() => setIsInfoModalOpen(false)}
      />
    </AppLayout>
  );
};

export default App;
