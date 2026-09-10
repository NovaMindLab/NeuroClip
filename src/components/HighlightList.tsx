import React from "react";
import { Flame, Clock, Award, Check, Sparkles } from "lucide-react";

export interface HighlightItem {
  id: number;
  start_time: number;
  end_time: number;
  duration: number;
  score: number;
  burst_ratio: number;
  emotion_tags: string[];
  start_snapped: boolean;
  end_snapped: boolean;
  summary: string;
}

interface HighlightListProps {
  highlights: HighlightItem[];
  selectedId: number;
  onSelectHighlight: (id: number) => void;
}

export const HighlightList: React.FC<HighlightListProps> = ({
  highlights,
  selectedId,
  onSelectHighlight,
}) => {
  const formatTime = (sec: number) => {
    const m = Math.floor(sec / 60);
    const s = Math.floor(sec % 60);
    return `${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
  };

  return (
    <div className="bg-[#10141e]/90 border border-[#1e2433] rounded-2xl p-5 shadow-lg flex flex-col h-full">
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <Award className="w-4 h-4 text-amber-400" />
          <span className="text-xs font-bold uppercase tracking-wider text-slate-200">
            高价值切片候选 ({highlights.length})
          </span>
        </div>
        <span className="text-[11px] text-slate-400">已智能吸附转场 (15~45s)</span>
      </div>

      <div className="space-y-2.5 overflow-y-auto pr-1 flex-1 max-h-[380px]">
        {highlights.length === 0 ? (
          <div className="py-12 text-center text-xs text-slate-500">
            点击上方启动按钮，AI 将在此展示多模态挖掘的高光片段
          </div>
        ) : (
          highlights.map((item) => {
            const isSelected = item.id === selectedId;

            return (
              <div
                key={item.id}
                onClick={() => onSelectHighlight(item.id)}
                className={`p-3.5 rounded-xl border cursor-pointer transition-all duration-200 text-left ${
                  isSelected
                    ? "bg-cyan-950/40 border-cyan-400/80 shadow-[0_0_15px_rgba(0,240,255,0.15)] ring-1 ring-cyan-400/30"
                    : "bg-[#0d1017]/70 border-slate-800 hover:border-slate-700 hover:bg-[#131824]"
                }`}
              >
                <div className="flex items-center justify-between mb-1.5">
                  <div className="flex items-center gap-2">
                    <span
                      className={`text-xs font-bold px-2 py-0.5 rounded ${
                        isSelected
                          ? "bg-cyan-500 text-slate-950"
                          : "bg-slate-800 text-slate-300"
                      }`}
                    >
                      #{item.id} 高光
                    </span>
                    <div className="flex items-center gap-1 text-[11px] text-amber-400 font-semibold">
                      <Flame className="w-3 h-3 fill-amber-400 text-amber-400" />
                      <span>{item.burst_ratio.toFixed(1)}x 能量</span>
                    </div>
                  </div>

                  <span className="text-[11px] font-mono text-cyan-300 font-semibold">
                    综合分: {(item.score * 100).toFixed(0)}
                  </span>
                </div>

                {/* Duration & Time range */}
                <div className="flex items-center gap-3 text-xs text-slate-300 mb-2">
                  <div className="flex items-center gap-1">
                    <Clock className="w-3.5 h-3.5 text-slate-400" />
                    <span className="font-mono">
                      {formatTime(item.start_time)} - {formatTime(item.end_time)}
                    </span>
                  </div>
                  <span className="text-slate-500">•</span>
                  <span className="font-mono font-medium text-slate-200">
                    {item.duration.toFixed(1)} 秒
                  </span>
                </div>

                {/* Tags and snap status */}
                <div className="flex flex-wrap items-center gap-1.5 mt-2">
                  {item.emotion_tags.map((tag, i) => (
                    <span
                      key={i}
                      className="px-2 py-0.5 rounded-full bg-purple-950/60 border border-purple-500/30 text-[10px] text-purple-300 flex items-center gap-1"
                    >
                      <Sparkles className="w-2.5 h-2.5" />
                      <span>{tag}</span>
                    </span>
                  ))}

                  {item.start_snapped && item.end_snapped && (
                    <span className="px-2 py-0.5 rounded-full bg-emerald-950/60 border border-emerald-500/30 text-[10px] text-emerald-300 flex items-center gap-1 ml-auto">
                      <Check className="w-2.5 h-2.5" />
                      <span>双向镜头对齐</span>
                    </span>
                  )}
                </div>
              </div>
            );
          })
        )}
      </div>
    </div>
  );
};
