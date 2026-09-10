import React from "react";
import { FolderSearch, Film, Download, Monitor, FolderPlus, Loader2 } from "lucide-react";

interface ScanControlBarProps {
  isScanning: boolean;
  scanProgressText: string;
  totalDiscovered: number;
  onScanPreset: (preset: "movies" | "downloads" | "desktop" | "all") => void;
  onScanCustom: () => void;
}

export const ScanControlBar: React.FC<ScanControlBarProps> = ({
  isScanning,
  scanProgressText,
  totalDiscovered,
  onScanPreset,
  onScanCustom,
}) => {
  return (
    <div className="rounded-2xl bg-gradient-to-b from-[#121622] to-[#0e121a] border border-[#1e2433] p-5 shadow-xl shadow-black/40 space-y-4">
      <div className="flex flex-wrap items-center justify-between gap-4">
        <div>
          <h2 className="text-sm font-bold text-white flex items-center gap-2">
            <FolderSearch className="w-4 h-4 text-cyan-400" />
            <span>极速扫描 PC 视频入库 (SQLite3 持久化)</span>
          </h2>
          <p className="text-xs text-slate-400 mt-0.5">
            采用 Rust 深度剪枝多线程引擎，跳过 node_modules 与系统垃圾，单事务批量入库
          </p>
        </div>

        {/* 快捷扫描按钮组 */}
        <div className="flex flex-wrap items-center gap-2">
          <button
            disabled={isScanning}
            onClick={() => onScanPreset("all")}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-gradient-to-r from-cyan-600 to-blue-600 hover:from-cyan-500 hover:to-blue-500 disabled:opacity-50 text-white text-xs font-semibold shadow-md shadow-cyan-600/20 transition-all active:scale-95"
          >
            {isScanning ? (
              <Loader2 className="w-3.5 h-3.5 animate-spin" />
            ) : (
              <FolderSearch className="w-3.5 h-3.5" />
            )}
            <span>一键全速扫描常用目录</span>
          </button>

          <button
            disabled={isScanning}
            onClick={() => onScanPreset("movies")}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-slate-800/80 hover:bg-slate-700/80 border border-slate-700 disabled:opacity-50 text-slate-300 text-xs font-medium transition-all"
          >
            <Film className="w-3.5 h-3.5 text-cyan-400" />
            <span>影视目录 (~/Movies)</span>
          </button>

          <button
            disabled={isScanning}
            onClick={() => onScanPreset("downloads")}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-slate-800/80 hover:bg-slate-700/80 border border-slate-700 disabled:opacity-50 text-slate-300 text-xs font-medium transition-all"
          >
            <Download className="w-3.5 h-3.5 text-purple-400" />
            <span>下载目录 (~/Downloads)</span>
          </button>

          <button
            disabled={isScanning}
            onClick={() => onScanPreset("desktop")}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-slate-800/80 hover:bg-slate-700/80 border border-slate-700 disabled:opacity-50 text-slate-300 text-xs font-medium transition-all"
          >
            <Monitor className="w-3.5 h-3.5 text-emerald-400" />
            <span>桌面录屏 (~/Desktop)</span>
          </button>

          <button
            disabled={isScanning}
            onClick={onScanCustom}
            className="flex items-center gap-1.5 px-3 py-1.5 rounded-xl bg-slate-800/80 hover:bg-slate-700/80 border border-slate-700 disabled:opacity-50 text-slate-300 text-xs font-medium transition-all"
          >
            <FolderPlus className="w-3.5 h-3.5 text-amber-400" />
            <span>自定义目录...</span>
          </button>
        </div>
      </div>

      {/* 扫描中动效与状态条 */}
      {isScanning && (
        <div className="pt-2 border-t border-slate-800/80 space-y-2">
          <div className="flex items-center justify-between text-xs">
            <span className="text-cyan-300 font-medium flex items-center gap-2">
              <Loader2 className="w-3.5 h-3.5 animate-spin text-cyan-400" />
              {scanProgressText || "正在深度扫描目录并提取元数据..."}
            </span>
            <span className="text-slate-400 font-mono">
              已发现视频: <strong className="text-cyan-300">{totalDiscovered}</strong> 个
            </span>
          </div>
          <div className="w-full h-1.5 bg-slate-800 rounded-full overflow-hidden">
            <div className="h-full bg-gradient-to-r from-cyan-400 via-blue-500 to-purple-500 rounded-full animate-pulse" />
          </div>
        </div>
      )}
    </div>
  );
};
