import React, { useState } from "react";
import { X, Play, Pause, Volume2, Download, FolderOpen, Check, Smartphone, Monitor, Sparkles } from "lucide-react";
import { CommentaryData } from "./CommentaryStudio";

interface ClipPlayerModalProps {
  isOpen: boolean;
  onClose: () => void;
  clipTitle: string;
  duration: number;
  commentary: CommentaryData | null;
  videoPath?: string;
  isVertical?: boolean;
}

export const ClipPlayerModal: React.FC<ClipPlayerModalProps> = ({
  isOpen,
  onClose,
  clipTitle,
  duration,
  commentary,
  videoPath = "neuroclip_exports/neuroclip_highlight_1.mp4",
  isVertical = true,
}) => {
  const [isPlaying, setIsPlaying] = useState(true);
  const [copied, setCopied] = useState(false);

  if (!isOpen) return null;

  const handleCopyPath = () => {
    navigator.clipboard.writeText(videoPath);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div className="bg-[#10141e] border border-[#1e2433] rounded-2xl w-full max-w-4xl max-h-[92vh] overflow-hidden shadow-2xl flex flex-col">
        {/* Modal Header */}
        <div className="px-6 py-3.5 border-b border-[#1e2433] flex items-center justify-between">
          <div className="flex items-center gap-2.5">
            <span className="text-sm font-bold text-slate-100">{clipTitle}</span>
            <span className="text-xs px-2 py-0.5 rounded bg-cyan-950 text-cyan-300 border border-cyan-500/30 flex items-center gap-1">
              {isVertical ? <Smartphone className="w-3 h-3" /> : <Monitor className="w-3 h-3" />}
              <span>{isVertical ? "9:16 竖屏短视频" : "16:9 横屏短视频"}</span>
            </span>
            <span className="text-xs text-slate-400 font-mono">
              {duration.toFixed(1)}s
            </span>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded-lg hover:bg-slate-800 text-slate-400 hover:text-slate-200 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Video Player Canvas Mockup */}
        <div className="relative flex-1 bg-black flex items-center justify-center overflow-hidden min-h-[420px] max-h-[560px]">
          {/* If 9:16 vertical mode, show blurred ambient background with centered mobile frame */}
          {isVertical ? (
            <div className="relative w-full h-full flex items-center justify-center overflow-hidden">
              {/* Blurred Ambient Background */}
              <div className="absolute inset-0 bg-gradient-to-br from-indigo-950 via-slate-900 to-cyan-950/40 filter blur-xl scale-125 opacity-70"></div>

              {/* Centered 9:16 Smartphone Mockup Screen */}
              <div className="relative aspect-[9/16] h-[94%] bg-[#0d1017] rounded-2xl border-2 border-cyan-500/40 shadow-[0_0_40px_rgba(0,240,255,0.25)] flex items-center justify-center overflow-hidden">
                {/* Internal Video Layer */}
                <div className="absolute inset-0 bg-[radial-gradient(#00f0ff_1px,transparent_1px)] [background-size:16px_16px] opacity-25"></div>

                {/* Central Play/Pause Toggle */}
                <button
                  onClick={() => setIsPlaying(!isPlaying)}
                  className="z-10 w-14 h-14 rounded-full bg-cyan-500/20 border border-cyan-400/50 backdrop-blur-md flex items-center justify-center text-cyan-300 hover:scale-110 transition-all shadow-[0_0_20px_rgba(0,240,255,0.4)]"
                >
                  {isPlaying ? <Pause className="w-6 h-6 fill-current" /> : <Play className="w-6 h-6 fill-current ml-1" />}
                </button>

                {/* ASS Kinetic Subtitle Display */}
                {commentary && (
                  <div className="absolute bottom-16 inset-x-4 z-20 text-center pointer-events-none">
                    <span className="inline-block px-3 py-1.5 rounded-lg bg-black/85 backdrop-blur-sm text-yellow-300 font-extrabold text-sm tracking-wide drop-shadow-[0_2px_4px_rgba(0,0,0,0.9)] border border-yellow-500/40 animate-pulse">
                      {commentary.subtitles[0]?.text || commentary.hook}
                    </span>
                  </div>
                )}
              </div>
            </div>
          ) : (
            // 16:9 Landscape Screen
            <div className="relative aspect-video w-full h-full bg-slate-950 flex items-center justify-center overflow-hidden">
              <div className="absolute inset-0 bg-[radial-gradient(#00f0ff_1px,transparent_1px)] [background-size:16px_16px] opacity-30"></div>
              <button
                onClick={() => setIsPlaying(!isPlaying)}
                className="z-10 w-16 h-16 rounded-full bg-cyan-500/20 border border-cyan-400/50 backdrop-blur-md flex items-center justify-center text-cyan-300 hover:scale-110 transition-all shadow-[0_0_20px_rgba(0,240,255,0.4)]"
              >
                {isPlaying ? <Pause className="w-7 h-7 fill-current" /> : <Play className="w-7 h-7 fill-current ml-1" />}
              </button>

              {commentary && (
                <div className="absolute bottom-8 inset-x-8 z-20 text-center pointer-events-none">
                  <span className="inline-block px-4 py-1.5 rounded-lg bg-black/80 backdrop-blur-sm text-yellow-300 font-bold text-base border border-yellow-500/30">
                    {commentary.subtitles[0]?.text || commentary.hook}
                  </span>
                </div>
              )}
            </div>
          )}

          {/* 3-Track Audio Ducking Badges */}
          <div className="absolute top-4 left-4 z-20 flex items-center gap-2">
            <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-black/75 backdrop-blur-md border border-slate-700 text-[11px]">
              <Volume2 className="w-3.5 h-3.5 text-cyan-400" />
              <span className="text-slate-300">原声闪避:</span>
              <span className="text-emerald-400 font-mono font-bold">-12dB</span>
            </div>
            <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-lg bg-black/75 backdrop-blur-md border border-purple-500/40 text-[11px]">
              <Sparkles className="w-3.5 h-3.5 text-purple-400" />
              <span className="text-purple-300">BGM卡点:</span>
              <span className="text-purple-400 font-mono font-bold">-15dB</span>
            </div>
          </div>
        </div>

        {/* Export Details & Actions */}
        <div className="p-4 bg-[#0d1017] border-t border-[#1e2433] flex flex-col md:flex-row items-center justify-between gap-4">
          <div className="text-left w-full md:w-auto">
            <div className="flex items-center gap-2 text-xs text-slate-400">
              <span>短视频成片路径:</span>
              <span className="font-mono text-slate-200 truncate max-w-md">{videoPath}</span>
            </div>
            <div className="text-[11px] text-slate-500 mt-1">
              纯视频成片 • {isVertical ? "1080x1920 (9:16 动态模糊)" : "1920x1080 (16:9)"} • ASS 动效字幕 • 3轨双闪避混音
            </div>
          </div>

          <div className="flex items-center gap-2.5 w-full md:w-auto justify-end">
            <button
              onClick={handleCopyPath}
              className="flex items-center gap-1.5 px-3 py-2 rounded-xl bg-slate-800 hover:bg-slate-700 text-slate-200 text-xs font-medium border border-slate-700 transition-colors"
            >
              {copied ? <Check className="w-3.5 h-3.5 text-emerald-400" /> : <FolderOpen className="w-3.5 h-3.5" />}
              <span>{copied ? "已复制路径" : "复制路径"}</span>
            </button>

            <button
              onClick={onClose}
              className="flex items-center gap-1.5 px-4 py-2 rounded-xl bg-gradient-to-r from-cyan-500 to-purple-600 hover:from-cyan-400 hover:to-purple-500 text-white text-xs font-semibold shadow-md shadow-cyan-500/20 transition-all"
            >
              <Download className="w-3.5 h-3.5" />
              <span>保存短视频成片</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
