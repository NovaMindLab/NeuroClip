import React from "react";
import { Zap, Scissors, Sparkles } from "lucide-react";

interface TimelineWaveformProps {
  totalDuration: number;
  rmsCurve: [number, number][]; // [time, rms]
  selectedStart: number;
  selectedEnd: number;
  onRangeChange?: (start: number, end: number) => void;
  transitions?: number[]; // cut timestamps
  emotionMarkers?: { time: number; label: string }[];
}

export const TimelineWaveform: React.FC<TimelineWaveformProps> = ({
  totalDuration,
  rmsCurve,
  selectedStart,
  selectedEnd,
  transitions = [12.0, 26.5, 41.0, 55.5, 70.0, 84.5, 99.0, 113.5, 128.0, 142.5, 157.0, 171.5],
  emotionMarkers = [
    { time: 24.5, label: "欢呼声" },
    { time: 48.0, label: "掌声" },
    { time: 118.0, label: "呐喊" },
  ],
}) => {
  const maxRms = Math.max(0.01, ...rmsCurve.map(([, r]) => r));

  const formatTime = (sec: number) => {
    const m = Math.floor(sec / 60);
    const s = Math.floor(sec % 60);
    return `${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
  };

  const startPercent = (selectedStart / totalDuration) * 100;
  const endPercent = (selectedEnd / totalDuration) * 100;
  const widthPercent = Math.max(2, endPercent - startPercent);

  return (
    <div className="bg-[#10141e]/90 border border-[#1e2433] rounded-2xl p-5 shadow-lg">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <Zap className="w-4 h-4 text-cyan-400" />
          <span className="text-xs font-bold uppercase tracking-wider text-slate-200">
            多模态时间轴 & 声学波形挖掘图
          </span>
        </div>

        <div className="flex items-center gap-4 text-xs">
          <div className="flex items-center gap-1.5 text-slate-400">
            <span className="w-2.5 h-2.5 rounded-sm bg-cyan-500/80"></span>
            <span>高能候选区间 ({formatTime(selectedStart)} - {formatTime(selectedEnd)})</span>
          </div>
          <div className="flex items-center gap-1.5 text-slate-400">
            <span className="w-2.5 h-0.5 bg-purple-400"></span>
            <span>TransNetV2 镜头 Cut 点</span>
          </div>
        </div>
      </div>

      {/* Waveform Canvas Container */}
      <div className="relative h-28 bg-[#0a0c12] rounded-xl border border-slate-800 overflow-hidden">
        {/* Selected Range Highlight Overlay */}
        <div
          className="absolute top-0 bottom-0 bg-cyan-500/15 border-x-2 border-cyan-400 z-10 pointer-events-none transition-all duration-200 backdrop-brightness-125"
          style={{
            left: `${startPercent}%`,
            width: `${widthPercent}%`,
          }}
        >
          <div className="absolute -top-0.5 left-1/2 -translate-x-1/2 px-2 py-0.5 bg-cyan-500 text-[10px] font-mono font-bold text-black rounded-b">
            {(selectedEnd - selectedStart).toFixed(1)}s
          </div>
        </div>

        {/* TransNetV2 Shot Cut Vertical Lines */}
        {transitions.map((t, idx) => {
          const cutPercent = (t / totalDuration) * 100;
          if (cutPercent > 100) return null;
          return (
            <div
              key={idx}
              className="absolute top-0 bottom-0 w-[1px] bg-purple-400/40 z-0 flex flex-col justify-end pointer-events-none"
              style={{ left: `${cutPercent}%` }}
            >
              <Scissors className="w-2.5 h-2.5 text-purple-400/80 -ml-1 mb-0.5" />
            </div>
          );
        })}

        {/* Emotion markers */}
        {emotionMarkers.map((m, idx) => {
          const markerPercent = (m.time / totalDuration) * 100;
          if (markerPercent > 100) return null;
          return (
            <div
              key={idx}
              className="absolute top-2 z-20 flex items-center gap-1 px-1.5 py-0.5 rounded bg-purple-950/80 border border-purple-500/40 text-[9px] text-purple-300 font-medium transform -translate-x-1/2"
              style={{ left: `${markerPercent}%` }}
            >
              <Sparkles className="w-2.5 h-2.5 text-purple-300" />
              <span>{m.label}</span>
            </div>
          );
        })}

        {/* Audio RMS Waveform Bars */}
        <div className="absolute inset-0 flex items-end px-2 pb-2 pt-6 gap-[2px]">
          {rmsCurve.length > 0 ? (
            rmsCurve.map(([t, r], i) => {
              const heightRatio = Math.min(1, r / maxRms);
              const inRange = t >= selectedStart && t <= selectedEnd;
              const isSurge = r > maxRms * 0.6;

              return (
                <div
                  key={i}
                  className={`flex-1 rounded-t-sm transition-all duration-100 ${
                    inRange
                      ? isSurge
                        ? "bg-gradient-to-t from-cyan-500 to-yellow-300 shadow-[0_0_6px_rgba(34,211,238,0.8)]"
                        : "bg-cyan-400/90"
                      : isSurge
                      ? "bg-purple-500/70"
                      : "bg-slate-700/60"
                  }`}
                  style={{
                    height: `${Math.max(4, heightRatio * 85)}%`,
                  }}
                  title={`时间: ${formatTime(t)} | RMS: ${r.toFixed(3)}`}
                />
              );
            })
          ) : (
            <div className="w-full h-full flex items-center justify-center text-xs text-slate-500">
              加载音频能量特征中...
            </div>
          )}
        </div>
      </div>

      {/* Timeline Time Labels */}
      <div className="flex justify-between text-[11px] font-mono text-slate-500 mt-2 px-1">
        <span>00:00</span>
        <span>{formatTime(totalDuration * 0.25)}</span>
        <span>{formatTime(totalDuration * 0.5)}</span>
        <span>{formatTime(totalDuration * 0.75)}</span>
        <span>{formatTime(totalDuration)}</span>
      </div>
    </div>
  );
};
