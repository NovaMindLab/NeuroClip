import React from "react";
import { ShieldCheck, Sparkles, Eye, Film, Volume2, Cpu } from "lucide-react";
import { AppNavTab } from "../../types/library";

interface WorkspaceHeaderProps {
  activeTab: AppNavTab;
  ffmpegReady: boolean;
  isWatcherActive: boolean;
  onToggleWatcher: () => void;
  onOpenInfoModal: () => void;
}

export const WorkspaceHeader: React.FC<WorkspaceHeaderProps> = ({
  activeTab,
  ffmpegReady,
  isWatcherActive,
  onToggleWatcher,
  onOpenInfoModal,
}) => {
  const titles: Record<AppNavTab, { title: string; subtitle: string }> = {
    studio: {
      title: "高光剪辑工作台",
      subtitle: "多模态音视频爆点挖掘 ➔ 智能人设文案 ➔ 9:16 工业级侧链混流",
    },
    library: {
      title: "PC 本地视频资产库",
      subtitle: "高性能多目录递归扫描 ➔ SQLite3 毫秒级入库 ➔ 一键直达工作台",
    },
    watcher: {
      title: "无人值守自动化监听中台",
      subtitle: "文件夹防死锁稳定性探测 ➔ 新增视频后台全自动流水线处理",
    },
    settings: {
      title: "系统配置与硬件加速",
      subtitle: "VideoToolbox / NVENC / QSV 编解码管道与端侧模型权重状态",
    },
  };

  const currentMeta = titles[activeTab] || titles.studio;

  return (
    <header className="h-16 border-b border-[#1e2433] bg-[#0d1118]/80 backdrop-blur-md px-6 flex items-center justify-between z-20 shrink-0">
      {/* 视图标题与副标 */}
      <div className="flex flex-col">
        <div className="flex items-center gap-2">
          <h1 className="text-sm font-bold text-white tracking-wide">{currentMeta.title}</h1>
          <span className="text-xs text-slate-400">/</span>
          <span className="text-xs text-cyan-400 font-mono">Workspace</span>
        </div>
        <p className="text-[11px] text-slate-400 truncate max-w-xl">{currentMeta.subtitle}</p>
      </div>

      {/* 右侧状态指示灯与快捷功能 */}
      <div className="flex items-center gap-3">
        {/* Model status badges */}
        <div className="hidden xl:flex items-center gap-2">
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

        {/* Folder Watcher toggle button */}
        <button
          onClick={onToggleWatcher}
          className={`flex items-center gap-1.5 px-3 py-1.5 rounded-lg border text-xs font-semibold transition-all ${
            isWatcherActive
              ? "bg-emerald-950/60 border-emerald-400 text-emerald-300 shadow-[0_0_12px_rgba(16,185,129,0.3)]"
              : "bg-slate-900 hover:bg-slate-800 border-slate-700 text-slate-300"
          }`}
        >
          <Eye className={`w-3.5 h-3.5 ${isWatcherActive ? "text-emerald-400 animate-pulse" : "text-slate-400"}`} />
          <span>{isWatcherActive ? "监听运行中" : "监听开关"}</span>
        </button>

        {/* FFmpeg 状态 */}
        <div className="hidden sm:flex items-center gap-2 px-3 py-1.5 rounded-lg bg-[#121620] border border-slate-800 text-xs">
          <ShieldCheck className={`w-4 h-4 ${ffmpegReady ? "text-emerald-400" : "text-amber-400"}`} />
          <span className="text-slate-300">FFmpeg:</span>
          <span className={ffmpegReady ? "text-emerald-400 font-medium" : "text-amber-400 font-medium"}>
            {ffmpegReady ? "就绪" : "回退"}
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
