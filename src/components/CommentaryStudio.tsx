import React, { useState } from "react";
import { FileText, Sparkles, Volume2, Play, RefreshCw, Layers, Music, Flame, HelpCircle, Smile, BookOpen } from "lucide-react";

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
  currentPersona: string;
  onChangePersona: (p: string) => void;
  currentBgm: string;
  onChangeBgm: (b: string) => void;
}

export const CommentaryStudio: React.FC<CommentaryStudioProps> = ({
  data,
  onUpdateCommentary,
  onPreviewTts,
  onRegenerate,
  onOpenPlayer,
  currentPersona,
  onChangePersona,
  currentBgm,
  onChangeBgm,
}) => {
  const [isPlayingTts, setIsPlayingTts] = useState(false);

  if (!data) {
    return (
      <div className="bg-[#10141e]/90 border border-[#1e2433] rounded-2xl p-6 shadow-lg flex items-center justify-center text-xs text-slate-500 min-h-[300px]">
        选择高光片段后，此处将呈现前3秒黄金Hook、严格配速解说词与双语字幕轴
      </div>
    );
  }

  const personaRates: Record<string, number> = {
    esports: 5.0,
    suspense: 3.8,
    roast: 4.0,
    breakdown: 4.2,
  };

  const targetChars = Math.round(data.duration_seconds * (personaRates[currentPersona] || 4.2));
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
      {/* Header with Title & Refresh */}
      <div className="flex items-center justify-between mb-3">
        <div className="flex items-center gap-2">
          <FileText className="w-4 h-4 text-cyan-400" />
          <span className="text-xs font-bold uppercase tracking-wider text-slate-200">
            短视频解说工坊 (多流派矩阵 & ASS 动效字幕)
          </span>
        </div>

        <div className="flex items-center gap-2">
          <span
            className={`px-2 py-0.5 rounded text-[11px] font-mono font-medium border ${
              isPaceGood
                ? "bg-emerald-950/60 text-emerald-300 border-emerald-500/30"
                : "bg-amber-950/60 text-amber-300 border-amber-500/30"
            }`}
          >
            字数: {currentChars} / 目标: {targetChars} ({personaRates[currentPersona]}字/秒)
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

      {/* 4-Persona Segmented Tabs */}
      <div className="grid grid-cols-4 gap-1.5 p-1 bg-[#0a0c12] rounded-xl border border-slate-800 mb-3">
        {[
          { id: "esports", name: "激情电竞", rate: "5.0字/s", icon: <Flame className="w-3 h-3 text-orange-400" /> },
          { id: "suspense", name: "悬念反转", rate: "3.8字/s", icon: <HelpCircle className="w-3 h-3 text-purple-400" /> },
          { id: "roast", name: "幽默下饭", rate: "4.0字/s", icon: <Smile className="w-3 h-3 text-yellow-400" /> },
          { id: "breakdown", name: "硬核科普", rate: "4.2字/s", icon: <BookOpen className="w-3 h-3 text-sky-400" /> },
        ].map((p) => {
          const isActive = currentPersona === p.id;
          return (
            <button
              key={p.id}
              onClick={() => onChangePersona(p.id)}
              className={`flex flex-col items-center py-1.5 px-1 rounded-lg text-[11px] font-medium transition-all ${
                isActive
                  ? "bg-slate-800 text-cyan-300 border border-cyan-500/40 shadow-sm"
                  : "text-slate-400 hover:text-slate-200"
              }`}
            >
              <div className="flex items-center gap-1">
                {p.icon}
                <span>{p.name}</span>
              </div>
              <span className="text-[9px] text-slate-500">{p.rate}</span>
            </button>
          );
        })}
      </div>

      {/* 3-Second Golden Hook Alert Card */}
      <div className="mb-3 p-2.5 rounded-xl bg-gradient-to-r from-amber-500/10 via-cyan-500/10 to-purple-500/10 border border-amber-500/30">
        <div className="flex items-center gap-1.5 text-[10px] font-bold text-amber-400 mb-0.5">
          <Sparkles className="w-3 h-3" />
          <span>前 3 秒黄金悬念 Hook (0.0s - 3.0s)</span>
        </div>
        <p className="text-xs font-semibold text-slate-100 italic">
          "{data.hook}"
        </p>
      </div>

      {/* Full script text editor */}
      <div className="mb-3">
        <label className="text-[11px] font-medium text-slate-400 mb-1 block">
          完整解说词正文:
        </label>
        <textarea
          rows={2}
          value={data.full_commentary}
          onChange={(e) => onUpdateCommentary && onUpdateCommentary(e.target.value)}
          className="w-full bg-[#0a0c12] border border-slate-800 rounded-xl p-2.5 text-xs text-slate-200 focus:outline-none focus:border-cyan-500 transition-colors resize-none leading-relaxed"
        />
      </div>

      {/* BGM & Kinetic Subtitle Badges */}
      <div className="flex items-center justify-between p-2 rounded-xl bg-[#0a0c12] border border-slate-800 mb-3 text-xs">
        <div className="flex items-center gap-2">
          <Music className="w-3.5 h-3.5 text-purple-400" />
          <span className="text-slate-400 text-[11px]">伴奏卡点:</span>
          <select
            value={currentBgm}
            onChange={(e) => onChangeBgm(e.target.value)}
            className="bg-slate-900 border border-slate-700 text-slate-200 rounded px-2 py-0.5 text-xs focus:outline-none"
          >
            <option value="trap">燃系电竞 Trap (140BPM)</option>
            <option value="drone">悬念紧迫心跳 (90BPM)</option>
            <option value="meme">幽默反转搞怪 (120BPM)</option>
            <option value="none">无背景音乐</option>
          </select>
        </div>

        <div className="flex items-center gap-1 text-[11px] text-cyan-400 bg-cyan-950/60 px-2 py-0.5 rounded border border-cyan-500/30">
          <Sparkles className="w-3 h-3" />
          <span>ASS 逐字弹性跳动动效字幕</span>
        </div>
      </div>

      {/* Subtitles Cue Preview */}
      <div className="flex-1 flex flex-col min-h-0">
        <div className="flex items-center justify-between mb-1">
          <span className="text-[11px] font-medium text-slate-400 flex items-center gap-1">
            <Layers className="w-3 h-3 text-purple-400" />
            <span>逐句字幕时间轴 (共 {data.subtitles.length} 条):</span>
          </span>
        </div>

        <div className="bg-[#0a0c12] rounded-xl border border-slate-800 p-2 overflow-y-auto space-y-1 max-h-[110px] text-xs">
          {data.subtitles.map((sub, i) => (
            <div
              key={i}
              className="flex items-center gap-2 p-1 rounded-lg bg-slate-900/50 hover:bg-slate-900 transition-colors"
            >
              <span className="text-[10px] font-mono text-cyan-400 shrink-0 w-20">
                {sub.start_sec.toFixed(1)}s → {sub.end_sec.toFixed(1)}s
              </span>
              <span className="text-slate-300 truncate text-[11px]">{sub.text}</span>
            </div>
          ))}
        </div>
      </div>

      {/* Action Footer */}
      <div className="mt-3 pt-2.5 border-t border-slate-800 flex items-center justify-between">
        <div className="flex items-center gap-2 text-[11px] text-slate-400">
          <Volume2 className="w-3.5 h-3.5 text-cyan-400" />
          <span>原声闪避 <strong className="text-cyan-300">-12dB</strong> / BGM 闪避 <strong className="text-purple-300">-15dB</strong></span>
        </div>

        <div className="flex items-center gap-2">
          <button
            onClick={handleTtsPlay}
            disabled={isPlayingTts}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium border border-slate-700 transition-all"
          >
            <Volume2 className={`w-3.5 h-3.5 ${isPlayingTts ? "text-cyan-400 animate-pulse" : ""}`} />
            <span>{isPlayingTts ? "播放中..." : "试听旁白"}</span>
          </button>

          <button
            onClick={onOpenPlayer}
            className="flex items-center gap-1.5 px-4 py-1.5 rounded-lg bg-cyan-500 hover:bg-cyan-400 text-slate-950 font-semibold text-xs shadow-md shadow-cyan-500/20 transition-all active:scale-95"
          >
            <Play className="w-3.5 h-3.5 fill-current" />
            <span>预览混剪短视频</span>
          </button>
        </div>
      </div>
    </div>
  );
};
