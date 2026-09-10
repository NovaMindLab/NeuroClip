import React from "react";
import { Mic, Activity, FileText, AudioWaveform, Sliders, Check, Loader2 } from "lucide-react";

export interface PipelineStepState {
  id: string;
  title: string;
  subtitle: string;
  icon: React.ReactNode;
  status: "idle" | "running" | "done";
}

interface PipelineStepperProps {
  currentStep: string;
  progress: number;
  statusMessage: string;
}

export const PipelineStepper: React.FC<PipelineStepperProps> = ({
  currentStep,
  progress,
  statusMessage,
}) => {
  const steps: PipelineStepState[] = [
    {
      id: "extracting_audio",
      title: "Step 1: 音画抽取",
      subtitle: "16kHz 单声道 WAV + 抽帧",
      icon: <Mic className="w-4 h-4" />,
      status:
        currentStep === "extracting_audio"
          ? "running"
          : progress > 0.15
          ? "done"
          : "idle",
    },
    {
      id: "analyzing_highlights",
      title: "Step 2: 双通道高光挖掘",
      subtitle: "RMS 滑窗 + YAMNet + TransNetV2",
      icon: <Activity className="w-4 h-4" />,
      status:
        currentStep === "analyzing_highlights"
          ? "running"
          : progress > 0.40
          ? "done"
          : "idle",
    },
    {
      id: "generating_commentary",
      title: "Step 3: 智能解说生成",
      subtitle: "黄金 Hook + 4.2字/秒 对齐",
      icon: <FileText className="w-4 h-4" />,
      status:
        currentStep === "generating_commentary"
          ? "running"
          : progress > 0.60
          ? "done"
          : "idle",
    },
    {
      id: "synthesizing_tts",
      title: "Step 4: 离线 TTS 旁白",
      subtitle: "Sherpa-ONNX 语音合成",
      icon: <AudioWaveform className="w-4 h-4" />,
      status:
        currentStep === "synthesizing_tts"
          ? "running"
          : progress > 0.80
          ? "done"
          : "idle",
    },
    {
      id: "rendering_video",
      title: "Step 5: 闪避混音与硬字幕",
      subtitle: "sidechain -12dB + subtitles",
      icon: <Sliders className="w-4 h-4" />,
      status:
        currentStep === "rendering_video"
          ? "running"
          : progress >= 1.0
          ? "done"
          : "idle",
    },
  ];

  return (
    <div className="bg-[#10141e]/90 border border-[#1e2433] rounded-2xl p-5 shadow-lg">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <span className="text-xs font-bold uppercase tracking-wider text-cyan-400">
            NeuroClip 自动化端侧流水线
          </span>
          <span className="text-xs text-slate-400">•</span>
          <span className="text-xs text-slate-300 font-mono">
            总体进度: {(progress * 100).toFixed(0)}%
          </span>
        </div>
        <span className="text-xs text-slate-400 truncate max-w-sm text-right">
          {statusMessage || "等待启动任务..."}
        </span>
      </div>

      {/* Progress Bar */}
      <div className="w-full h-1.5 bg-slate-800 rounded-full overflow-hidden mb-5">
        <div
          className="h-full bg-gradient-to-r from-cyan-400 via-sky-400 to-purple-500 transition-all duration-300 rounded-full shadow-[0_0_12px_rgba(0,240,255,0.6)]"
          style={{ width: `${Math.min(100, Math.max(0, progress * 100))}%` }}
        ></div>
      </div>

      {/* 5-Step Grid */}
      <div className="grid grid-cols-1 md:grid-cols-5 gap-3">
        {steps.map((step) => {
          const isDone = step.status === "done";
          const isRunning = step.status === "running";

          return (
            <div
              key={step.id}
              className={`flex flex-col p-3 rounded-xl border transition-all ${
                isRunning
                  ? "bg-cyan-950/40 border-cyan-400/60 shadow-[0_0_15px_rgba(0,240,255,0.15)] ring-1 ring-cyan-400/40"
                  : isDone
                  ? "bg-slate-900/60 border-emerald-500/30 text-slate-200"
                  : "bg-slate-900/20 border-slate-800/80 text-slate-500"
              }`}
            >
              <div className="flex items-center justify-between mb-1.5">
                <div
                  className={`p-1.5 rounded-lg ${
                    isRunning
                      ? "bg-cyan-500/20 text-cyan-300"
                      : isDone
                      ? "bg-emerald-500/20 text-emerald-400"
                      : "bg-slate-800 text-slate-500"
                  }`}
                >
                  {step.icon}
                </div>
                {isRunning && <Loader2 className="w-3.5 h-3.5 text-cyan-400 animate-spin" />}
                {isDone && <Check className="w-3.5 h-3.5 text-emerald-400 stroke-[3]" />}
              </div>
              <span className="text-xs font-semibold tracking-tight text-slate-100">
                {step.title}
              </span>
              <span className="text-[11px] text-slate-400 mt-0.5 leading-tight">
                {step.subtitle}
              </span>
            </div>
          );
        })}
      </div>
    </div>
  );
};
