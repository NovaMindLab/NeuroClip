import React from "react";
import { Film, Trash2, FileVideo } from "lucide-react";
import { MediaAssetItem } from "../../types/library";
import { QuickClipActionBtn } from "./QuickClipActionBtn";

interface AssetTableViewProps {
  assets: MediaAssetItem[];
  onLoadIntoStudio: (asset: MediaAssetItem) => void;
  onDeleteAsset?: (id: number | string) => void;
}

export const AssetTableView: React.FC<AssetTableViewProps> = ({
  assets,
  onLoadIntoStudio,
  onDeleteAsset,
}) => {
  if (assets.length === 0) {
    return (
      <div className="py-16 text-center rounded-2xl bg-[#121622] border border-[#1e2433] space-y-3">
        <FileVideo className="w-10 h-10 text-slate-400 mx-auto" />
        <p className="text-sm text-slate-300">暂无符合条件的本地视频资产</p>
        <p className="text-xs text-slate-400">点击上方「极速扫描」按钮，或指定文件夹快速入库</p>
      </div>
    );
  }

  const getFormatBadgeColor = (fmt: string) => {
    switch (fmt.toLowerCase()) {
      case "mp4":
        return "bg-cyan-950/80 text-cyan-300 border-cyan-500/30";
      case "mov":
        return "bg-purple-950/80 text-purple-300 border-purple-500/30";
      case "mkv":
        return "bg-sky-950/80 text-sky-300 border-sky-500/30";
      case "flv":
        return "bg-amber-950/80 text-amber-300 border-amber-500/30";
      default:
        return "bg-slate-800 text-slate-300 border-slate-700";
    }
  };

  return (
    <div className="rounded-2xl bg-[#121622] border border-[#1e2433] overflow-hidden shadow-xl">
      <div className="overflow-x-auto">
        <table className="w-full text-left text-xs">
          <thead className="bg-[#0e121a] text-slate-400 border-b border-[#1e2433] uppercase font-mono tracking-wider text-[11px]">
            <tr>
              <th className="py-3.5 px-4">视频名称</th>
              <th className="py-3.5 px-4">格式</th>
              <th className="py-3.5 px-4">文件体积</th>
              <th className="py-3.5 px-4">视频时长</th>
              <th className="py-3.5 px-4">分辨率</th>
              <th className="py-3.5 px-4">入库时间</th>
              <th className="py-3.5 px-4 text-right">工业切片操作</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-slate-800/60">
            {assets.map((asset) => (
              <tr
                key={asset.id}
                className="hover:bg-slate-800/40 transition-colors group cursor-pointer"
                onClick={() => onLoadIntoStudio(asset)}
              >
                {/* 名称与图标 */}
                <td className="py-3.5 px-4">
                  <div className="flex items-center gap-3">
                    <div className="w-8 h-8 rounded-lg bg-[#0a0c10] border border-slate-700/80 flex items-center justify-center text-cyan-400 shrink-0">
                      <Film className="w-4 h-4" />
                    </div>
                    <div className="max-w-md truncate">
                      <div className="font-semibold text-slate-200 group-hover:text-cyan-300 transition-colors truncate">
                        {asset.name}
                      </div>
                      <div className="text-[10px] text-slate-400 font-mono truncate" title={asset.fullPath}>
                        {asset.fullPath}
                      </div>
                    </div>
                  </div>
                </td>

                {/* 格式 */}
                <td className="py-3.5 px-4">
                  <span
                    className={`px-2 py-0.5 rounded text-[10px] font-mono font-bold border uppercase ${getFormatBadgeColor(
                      asset.format
                    )}`}
                  >
                    {asset.format}
                  </span>
                </td>

                {/* 文件体积 */}
                <td className="py-3.5 px-4 font-mono text-slate-300">
                  {asset.fileSizeFormatted}
                </td>

                {/* 时长 */}
                <td className="py-3.5 px-4 font-mono text-slate-300">
                  {asset.durationFormatted || "待探测"}
                </td>

                {/* 分辨率 */}
                <td className="py-3.5 px-4 font-mono text-slate-400 text-[11px]">
                  {asset.resolution || "1080P"}
                </td>

                {/* 入库时间 */}
                <td className="py-3.5 px-4 text-slate-400 text-[11px]">
                  {new Date(asset.createdAt * 1000).toLocaleDateString()}
                </td>

                {/* 操作栏 */}
                <td className="py-3.5 px-4 text-right">
                  <div className="flex items-center justify-end gap-2" onClick={(e) => e.stopPropagation()}>
                    <QuickClipActionBtn asset={asset} onLoad={onLoadIntoStudio} compact={true} />
                    {onDeleteAsset && (
                      <button
                        onClick={() => onDeleteAsset(asset.id)}
                        className="p-1.5 rounded-lg text-slate-400 hover:text-red-400 hover:bg-red-950/30 transition-colors"
                        title="从媒体库移除记录"
                      >
                        <Trash2 className="w-3.5 h-3.5" />
                      </button>
                    )}
                  </div>
                </td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>
    </div>
  );
};
