import React, { useState } from "react";
import { X, Play, Pause, Volume2, Download, FolderOpen, Check } from "lucide-react";
import { CommentaryData } from "./CommentaryStudio";

interface ClipPlayerModalProps {
  isOpen: boolean;
  onClose: () => void;
  clipTitle: string;
  duration: number;
  commentary: CommentaryData | null;
  videoPath?: string;
}

export const ClipPlayerModal: React.FC<ClipPlayerModalProps> = ({
  isOpen,
  onClose,
  clipTitle,
  duration,
  commentary,
  videoPath = "neuroclip_exports/neuroclip_highlight_1.mp4",
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
      <div className="bg-[#10141e] border border-[#1e2433] rounded-2xl w-full max-w-3xl overflow-hidden shadow-2xl flex flex-col">
        {/* Modal Header */}
        <div className="px-6 py-4 border-b border-[#1e2433] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <span className="text-sm font-bold text-slate-100">{clipTitle}</span>
            <span className="text-xs px-2 py-0.5 rounded bg-cyan-950 text-cyan-300 border border-cyan-500/30">
              {duration.toFixed(1)}s 高清短视频
            </span>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded-lg hover:bg-slate-800 text-slate-400 hover:text-slate-200 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        {/* Video Player Canvas Mockup with Burnt Subtitles */}
        <div className="relative aspect-video bg-black flex items-center justify-center overflow-hidden group">
          {/* Simulated Gameplay / Video Background */}
          <div className="absolute inset-0 bg-gradient-to-tr from-slate-900 via-indigo-950/40 to-slate-900 flex items-center justify-center">
            <div className="w-full h-full opacity-30 bg-[radial-gradient(#00f0ff_1px,transparent_1px)] [background-size:16px_16px]"></div>
          </div>

          {/* Central Play/Pause Toggle */}
          <button
            onClick={() => setIsPlaying(!isPlaying)}
            className="z-10 w-16 h-16 rounded-full bg-cyan-500/20 border border-cyan-400/50 backdrop-blur-md flex items-center justify-center text-cyan-300 hover:scale-110 transition-all shadow-[0_0_20px_rgba(0,240,255,0.4)]"
          >
            {isPlaying ? <Pause className="w-7 h-7 fill-current" /> : <Play className="w-7 h-7 fill-current ml-1" />}
          </button>

          {/* Audio Ducking Indicator Bar on top */}
          <div className="absolute top-4 left-4 z-20 flex items-center gap-2 px-3 py-1.5 rounded-lg bg-black/60 backdrop-blur-md border border-slate-700 text-xs">
            <Volume2 className="w-4 h-4 text-cyan-400" />
            <span className="text-slate-300">原片音量:</span>
            <span className="text-emerald-400 font-mono font-bold animate-pulse">-12dB (闪避中)</span>
          </div>

          {/* Burnt Hardsub Subtitle Display */}
          {commentary && (
            <div className="absolute bottom-6 inset-x-8 z-20 text-center pointer-events-none">
              <span className="inline-block px-4 py-1.5 rounded-lg bg-black/75 backdrop-blur-sm text-yellow-300 font-bold text-base md:text-lg tracking-wide drop-shadow-[0_2px_4px_rgba(0,0,0,0.8)] border border-yellow-500/30">
                {commentary.subtitles[0]?.text || commentary.hook}
              </span>
            </div>
          )}

          {/* Bottom scrub bar */}
          <div className="absolute bottom-0 inset-x-0 h-1 bg-slate-800">
            <div className="h-full bg-cyan-400 w-2/3 shadow-[0_0_8px_rgba(0,240,255,0.8)]"></div>
          </div>
        </div>

        {/* Export Details & Actions */}
        <div className="p-5 bg-[#0d1017] border-t border-[#1e2433] flex flex-col md:flex-row items-center justify-between gap-4">
          <div className="text-left w-full md:w-auto">
            <div className="flex items-center gap-2 text-xs text-slate-400">
              <span>文件导出位置:</span>
              <span className="font-mono text-slate-200 truncate max-w-sm">{videoPath}</span>
            </div>
            <div className="text-[11px] text-slate-500 mt-1">
              编码: H.264 / AAC 192k • 1080p • 硬字幕烧录 • Sidechain 自动闪避
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
              <span>保存导出成片</span>
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};
