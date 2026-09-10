import React from "react";
import { Film } from "lucide-react";
import { MediaAssetItem } from "../../types/library";
import { QuickClipActionBtn } from "./QuickClipActionBtn";

interface AssetGridCardProps {
  asset: MediaAssetItem;
  onLoadIntoStudio: (asset: MediaAssetItem) => void;
}

export const AssetGridCard: React.FC<AssetGridCardProps> = ({ asset, onLoadIntoStudio }) => {
  return (
    <div
      onClick={() => onLoadIntoStudio(asset)}
      className="group rounded-2xl bg-[#121622] border border-[#1e2433] hover:border-cyan-500/50 p-4 transition-all duration-300 hover:shadow-xl hover:shadow-cyan-500/10 cursor-pointer flex flex-col justify-between"
    >
      {/* 顶部模拟视频缩略图框 */}
      <div className="relative aspect-video rounded-xl bg-gradient-to-br from-slate-900 via-[#10141e] to-slate-950 border border-slate-800 flex items-center justify-center overflow-hidden mb-3">
        {/* 动态网格背景 */}
        <div className="absolute inset-0 bg-[radial-gradient(#1e293b_1px,transparent_1px)] [background-size:12px_12px] opacity-40" />

        <div className="w-12 h-12 rounded-full bg-cyan-950/60 border border-cyan-500/30 flex items-center justify-center text-cyan-400 group-hover:scale-110 transition-transform shadow-lg">
          <Film className="w-5 h-5" />
        </div>

        {/* 角标 */}
        <div className="absolute top-2 left-2 px-2 py-0.5 rounded bg-black/70 backdrop-blur-sm text-[10px] font-mono font-bold text-cyan-300 border border-white/10 uppercase">
          {asset.format}
        </div>

        <div className="absolute bottom-2 right-2 px-2 py-0.5 rounded bg-black/70 backdrop-blur-sm text-[10px] font-mono text-slate-200 border border-white/10">
          {asset.durationFormatted || "待探测"}
        </div>
      </div>

      {/* 资产信息 */}
      <div className="space-y-1.5 mb-4">
        <h3
          className="text-xs font-bold text-slate-200 group-hover:text-cyan-300 transition-colors truncate"
          title={asset.name}
        >
          {asset.name}
        </h3>
        <p className="text-[10px] text-slate-400 font-mono truncate" title={asset.fullPath}>
          {asset.fullPath}
        </p>

        <div className="flex items-center justify-between text-[11px] text-slate-400 pt-1 font-mono">
          <span>{asset.fileSizeFormatted}</span>
          <span>{asset.resolution || "1080P 60FPS"}</span>
        </div>
      </div>

      {/* 底部动作按钮 */}
      <div onClick={(e) => e.stopPropagation()}>
        <QuickClipActionBtn asset={asset} onLoad={onLoadIntoStudio} />
      </div>
    </div>
  );
};
