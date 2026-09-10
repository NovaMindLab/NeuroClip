import React from "react";
import { NeuroClipLogo } from "./NeuroClipLogo";
import { Cpu, Film, Volume2, ShieldCheck, Sparkles } from "lucide-react";

interface NavbarProps {
  ffmpegReady: boolean;
  onOpenInfoModal: () => void;
}

export const Navbar: React.FC<NavbarProps> = ({ ffmpegReady, onOpenInfoModal }) => {
  return (
    <header className="h-16 border-b border-[#1e2433] bg-[#0d1118]/80 backdrop-blur-md px-6 flex items-center justify-between z-20">
      <div className="flex items-center gap-6">
        <NeuroClipLogo size={36} showText={true} />
        
        {/* Model status badges */}
        <div className="hidden md:flex items-center gap-2 pl-4 border-l border-slate-800">
          <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-slate-900/90 border border-slate-800 text-[11px] text-slate-300">
            <Film className="w-3.5 h-3.5 text-cyan-400" />
            <span>TransNetV2</span>
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
          </div>

          <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-slate-900/90 border border-slate-800 text-[11px] text-slate-300">
            <Volume2 className="w-3.5 h-3.5 text-purple-400" />
            <span>YAMNet & RMS</span>
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
          </div>

          <div className="flex items-center gap-1.5 px-2.5 py-1 rounded-full bg-slate-900/90 border border-slate-800 text-[11px] text-slate-300">
            <Cpu className="w-3.5 h-3.5 text-sky-400" />
            <span>Sherpa-ONNX</span>
            <span className="w-1.5 h-1.5 rounded-full bg-emerald-400"></span>
          </div>
        </div>
      </div>

      <div className="flex items-center gap-3">
        <div className="flex items-center gap-2 px-3 py-1 rounded-lg bg-[#121620] border border-slate-800 text-xs">
          <ShieldCheck className={`w-4 h-4 ${ffmpegReady ? "text-emerald-400" : "text-amber-400"}`} />
          <span className="text-slate-300">FFmpeg 状态:</span>
          <span className={ffmpegReady ? "text-emerald-400 font-medium" : "text-amber-400 font-medium"}>
            {ffmpegReady ? "就绪 (原生管道)" : "智能回退模式"}
          </span>
        </div>

        <button
          onClick={onOpenInfoModal}
          className="flex items-center gap-1.5 px-3 py-1.5 rounded-lg bg-cyan-950/40 hover:bg-cyan-900/50 text-cyan-300 border border-cyan-500/30 text-xs font-medium transition-all"
        >
          <Sparkles className="w-3.5 h-3.5 text-cyan-400" />
          <span>架构规范</span>
        </button>
      </div>
    </header>
  );
};
