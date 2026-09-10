import React, { useState, useEffect } from "react";
import { Navbar } from "./components/Navbar";
import { VideoDropzone } from "./components/VideoDropzone";
import { PipelineStepper } from "./components/PipelineStepper";
import { TimelineWaveform } from "./components/TimelineWaveform";
import { HighlightList, HighlightItem } from "./components/HighlightList";
import { CommentaryStudio, CommentaryData } from "./components/CommentaryStudio";
import { ClipPlayerModal } from "./components/ClipPlayerModal";
import { ArchitectureModal } from "./components/ArchitectureModal";

export const App: React.FC = () => {
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

  // Mock initial highlight candidates based on multimodal fusion
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

  // Simulated RMS curve for visualization (180s video)
  const [rmsCurve, setRmsCurve] = useState<[number, number][]>([]);

  useEffect(() => {
    // Generate dynamic RMS waveform points
    const points: [number, number][] = [];
    for (let t = 0; t <= 180; t += 1.2) {
      let r = 0.05 + Math.sin(t * 0.2) * 0.02;
      // Surges around highlights
      if (t >= 16 && t <= 38) r += 0.35 * Math.abs(Math.sin(t * 0.8));
      if (t >= 70 && t <= 96) r += 0.28 * Math.abs(Math.cos(t * 0.6));
      if (t >= 124 && t <= 144) r += 0.24 * Math.abs(Math.sin(t * 0.9));
      points.push([t, r]);
    }
    setRmsCurve(points);
  }, []);

  const selectedHighlight = highlights.find((h) => h.id === selectedHighlightId) || highlights[0];

  // Commentary data for selected highlight
  const [commentaryMap, setCommentaryMap] = useState<Record<number, CommentaryData>>({
    1: {
      hook: "注意看！就在这千钧一发的瞬间！",
      full_commentary:
        "注意看！就在这千钧一发的瞬间！这场精彩对决突然发力，以不可思议的绝妙节奏瞬间扭转战局，行云流水般的掌控力让全场观众集体起立狂欢！不得不说，这一刻的精彩注定成为经典回放！",
      duration_seconds: 25.0,
      char_count: 105,
      subtitles: [
        { start_sec: 0.0, end_sec: 3.0, text: "注意看！就在这千钧一发的瞬间" },
        { start_sec: 3.0, end_sec: 8.5, text: "这场精彩对决突然发力" },
        { start_sec: 8.5, end_sec: 14.2, text: "以不可思议的绝妙节奏瞬间扭转战局" },
        { start_sec: 14.2, end_sec: 19.8, text: "行云流水般的掌控力让全场观众集体起立狂欢" },
        { start_sec: 19.8, end_sec: 25.0, text: "不得不说，这一刻的精彩注定成为经典回放" },
      ],
    },
    2: {
      hook: "千万别眨眼！这波操作堪称神级名场面！",
      full_commentary:
        "千万别眨眼！这波操作堪称神级名场面！选手展现出顶级的反应与爆发力，每一个细节都拿捏得恰到好处，堪称教科书级别的巅峰演绎！现场气氛直接拉满！这绝对是不可多得的高光时刻！",
      duration_seconds: 30.0,
      char_count: 126,
      subtitles: [
        { start_sec: 0.0, end_sec: 3.0, text: "千万别眨眼！这波操作堪称神级名场面" },
        { start_sec: 3.0, end_sec: 11.2, text: "选手展现出顶级的反应与爆发力" },
        { start_sec: 11.2, end_sec: 18.5, text: "每一个细节都拿捏得恰到好处" },
        { start_sec: 18.5, end_sec: 24.2, text: "堪称教科书级别的巅峰演绎" },
        { start_sec: 24.2, end_sec: 30.0, text: "现场气氛直接拉满，不可多得的高光时刻" },
      ],
    },
    3: {
      hook: "谁能想到！接下来这一幕彻底引爆全场！",
      full_commentary:
        "谁能想到！接下来这一幕彻底引爆全场！突然上演意料之外的幽默反转，现场气氛直接拉满，这波神来之笔让所有人忍俊不禁！这一刻的精彩注定成为经典回放！",
      duration_seconds: 24.0,
      char_count: 101,
      subtitles: [
        { start_sec: 0.0, end_sec: 3.0, text: "谁能想到！接下来这一幕彻底引爆全场" },
        { start_sec: 3.0, end_sec: 10.5, text: "突然上演意料之外的幽默反转" },
        { start_sec: 10.5, end_sec: 17.2, text: "这波神来之笔让所有人忍俊不禁" },
        { start_sec: 17.2, end_sec: 24.0, text: "这一刻的精彩注定成为经典回放" },
      ],
    },
  });

  const currentCommentary = commentaryMap[selectedHighlightId] || commentaryMap[1];

  const handleStartProcess = async () => {
    if (!selectedVideoPath) return;

    setIsProcessing(true);
    setProgress(0.05);
    setCurrentStep("extracting_audio");
    setStatusMessage("Step 1: FFmpeg 正在抽离单声道 16kHz WAV 音频...");

    // Try calling Tauri command if available
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
      });

      if (res?.highlights) {
        setHighlights(res.highlights);
      }

      unlisten();
    } catch {
      // Browser / Dev mock simulation runner
      const steps = [
        { step: "extracting_audio", p: 0.2, msg: "Step 1: 抽离 16kHz 单声道 WAV + 抽帧采样完成" },
        { step: "analyzing_highlights", p: 0.45, msg: "Step 2: 滑窗 RMS >2.5x 爆点挖掘 + YAMNet 情绪识别 + TransNetV2 镜头吸附完成" },
        { step: "generating_commentary", p: 0.65, msg: "Step 3: 前3秒黄金Hook锁定，4.2字/秒 对齐文案生成完成" },
        { step: "synthesizing_tts", p: 0.85, msg: "Step 4: Sherpa-ONNX 离线合成 16kHz voiceover.wav 旁白音轨完成" },
        { step: "rendering_video", p: 1.0, msg: "Step 5: sidechaincompress 闪避下压 12dB + subtitles 硬字幕压制完成！" },
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

  return (
    <div className="flex flex-col h-screen bg-[#0a0c10] text-slate-100 overflow-hidden font-sans">
      {/* Top Navbar */}
      <Navbar
        ffmpegReady={true}
        onOpenInfoModal={() => setIsInfoModalOpen(true)}
      />

      {/* Main Workspace Scroll Area */}
      <main className="flex-1 overflow-y-auto p-6 space-y-5">
        {/* Top Dropzone */}
        <VideoDropzone
          selectedVideoPath={selectedVideoPath}
          selectedVideoName={selectedVideoName}
          onVideoSelected={(p, n) => {
            setSelectedVideoPath(p);
            setSelectedVideoName(n);
          }}
          isProcessing={isProcessing}
          onStartProcess={handleStartProcess}
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
        <div className="grid grid-cols-1 lg:grid-cols-12 gap-5 pb-8">
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
            />
          </div>
        </div>
      </main>

      {/* Preview Modal */}
      <ClipPlayerModal
        isOpen={isPlayerModalOpen}
        onClose={() => setIsPlayerModalOpen(false)}
        clipTitle={`高光片段 #${selectedHighlight.id} - ${selectedHighlight.emotion_tags.join(" + ")}`}
        duration={selectedHighlight.duration}
        commentary={currentCommentary}
        videoPath={`/neuroclip_exports/neuroclip_highlight_${selectedHighlight.id}.mp4`}
      />

      {/* Architecture Modal */}
      <ArchitectureModal
        isOpen={isInfoModalOpen}
        onClose={() => setIsInfoModalOpen(false)}
      />
    </div>
  );
};

export default App;
