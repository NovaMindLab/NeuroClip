import React, { useState } from "react";
import { FileText, Sparkles, Volume2, Play, RefreshCw, Layers } from "lucide-react";

export interface SubtitleItem {
  start_sec: number;
  end_sec: number;
  text: string;
}

export interface CommentaryData {
  hook: string;
  full_commentary: string;
  duration_seconds: number;
  char_count: number;
  subtitles: SubtitleItem[];
}

interface CommentaryStudioProps {
  data: CommentaryData | null;
  onUpdateCommentary?: (newText: string) => void;
  onPreviewTts?: () => void;
  onRegenerate?: () => void;
  onOpenPlayer?: () => void;
}

export const CommentaryStudio: React.FC<CommentaryStudioProps> = ({
  data,
  onUpdateCommentary,
  onPreviewTts,
  onRegenerate,
  onOpenPlayer,
}) => {
  const [isPlayingTts, setIsPlayingTts] = useState(false);

  if (!data) {
    return (
      <div className="bg-[#10141e]/90 border border-[#1e2433] rounded-2xl p-6 shadow-lg flex items-center justify-center text-xs text-slate-500 min-h-[300px]">
        选择高光片段后，此处将呈现前3秒黄金Hook、严格配速解说词与双语字幕轴
      </div>
    );
  }

  const targetChars = Math.round(data.duration_seconds * 4.2);
  const currentChars = data.char_count;
  const paceDiff = currentChars - targetChars;
  const isPaceGood = Math.abs(paceDiff) <= 4;

  const handleTtsPlay = () => {
    setIsPlayingTts(true);
    if (onPreviewTts) onPreviewTts();
    setTimeout(() => setIsPlayingTts(false), 3000);
  };

  return (
    <div className="bg-[#10141e]/90 border border-[#1e2433] rounded-2xl p-5 shadow-lg flex flex-col h-full">
      <div className="flex items-center justify-between mb-4">
        <div className="flex items-center gap-2">
          <FileText className="w-4 h-4 text-cyan-400" />
          <span className="text-xs font-bold uppercase tracking-wider text-slate-200">
            解说工坊 (快节奏文案 & 4.2字/秒 对齐)
          </span>
        </div>

        {/* Word count & Pace meter */}
        <div className="flex items-center gap-2">
          <span
            className={`px-2 py-0.5 rounded text-[11px] font-mono font-medium border ${
              isPaceGood
                ? "bg-emerald-950/60 text-emerald-300 border-emerald-500/30"
                : "bg-amber-950/60 text-amber-300 border-amber-500/30"
            }`}
          >
            字数: {currentChars} / 目标: {targetChars} ({data.duration_seconds.toFixed(1)}s × 4.2字)
          </span>

          <button
            onClick={onRegenerate}
            title="重新生成文案"
            className="p-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-300 border border-slate-700 text-xs transition-colors"
          >
            <RefreshCw className="w-3.5 h-3.5" />
          </button>
        </div>
      </div>

      {/* 3-Second Golden Hook Alert Card */}
      <div className="mb-3 p-3 rounded-xl bg-gradient-to-r from-amber-500/10 via-cyan-500/10 to-purple-500/10 border border-amber-500/30">
        <div className="flex items-center gap-1.5 text-[11px] font-bold text-amber-400 mb-1">
          <Sparkles className="w-3.5 h-3.5" />
          <span>前 3 秒黄金悬念 Hook (已精准锁定时间轴 0.0s - 3.0s)</span>
        </div>
        <p className="text-sm font-semibold text-slate-100 italic">
          "{data.hook}"
        </p>
      </div>

      {/* Full script text editor */}
      <div className="mb-4">
        <label className="text-[11px] font-medium text-slate-400 mb-1 block">
          完整解说词正文:
        </label>
        <textarea
          rows={3}
          value={data.full_commentary}
          onChange={(e) => onUpdateCommentary && onUpdateCommentary(e.target.value)}
          className="w-full bg-[#0a0c12] border border-slate-800 rounded-xl p-3 text-xs text-slate-200 focus:outline-none focus:border-cyan-500 transition-colors resize-none leading-relaxed"
        />
      </div>

      {/* Subtitles Cue Preview */}
      <div className="flex-1 flex flex-col min-h-0">
        <div className="flex items-center justify-between mb-1.5">
          <span className="text-[11px] font-medium text-slate-400 flex items-center gap-1">
            <Layers className="w-3.5 h-3.5 text-purple-400" />
            <span>自动切分硬字幕时间轴 (SRT / ASS):</span>
          </span>
          <span className="text-[10px] text-slate-500">共 {data.subtitles.length} 条字幕</span>
        </div>

        <div className="bg-[#0a0c12] rounded-xl border border-slate-800 p-2.5 overflow-y-auto space-y-1.5 max-h-[140px] text-xs">
          {data.subtitles.map((sub, i) => (
            <div
              key={i}
              className="flex items-center gap-2 p-1.5 rounded-lg bg-slate-900/50 hover:bg-slate-900 transition-colors"
            >
              <span className="text-[10px] font-mono text-cyan-400 shrink-0 w-24">
                {sub.start_sec.toFixed(1)}s → {sub.end_sec.toFixed(1)}s
              </span>
              <span className="text-slate-300 truncate">{sub.text}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Action Footer */}
      <div className="mt-4 pt-3 border-t border-slate-800 flex items-center justify-between">
        <div className="flex items-center gap-2 text-xs text-slate-400">
          <Volume2 className="w-4 h-4 text-cyan-400" />
          <span>混音闪避: <strong className="text-cyan-300">-12dB (原声平滑恢复)</strong></span>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={handleTtsPlay}
            disabled={isPlayingTts}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium border border-slate-700 transition-all"
          >
            <Volume2 className={`w-3.5 h-3.5 ${isPlayingTts ? "text-cyan-400 animate-pulse" : ""}`} />
            <span>{isPlayingTts ? "播放中..." : "试听 TTS 旁白"}</span>
          </button>

          <button
            onClick={onOpenPlayer}
            className="flex items-center gap-1.5 px-4 py-1.5 rounded-lg bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-semibold text-xs shadow-md shadow-cyan-500/20 transition-all active:scale-95"
          >
            <Play className="w-3.5 h-3.5 fill-current" />
            <span>预览混剪成品</span>
          </button>
        </div>
      </div>
    </div>
  );
};
