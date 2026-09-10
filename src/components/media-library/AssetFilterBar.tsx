import React from "react";
import { Search, LayoutGrid, List, ArrowDownUp } from "lucide-react";

interface AssetFilterBarProps {
  searchQuery: string;
  onSearchChange: (q: string) => void;
  selectedFormat: string;
  onFormatChange: (fmt: string) => void;
  sortBy: string;
  onSortChange: (sort: string) => void;
  viewMode: "table" | "grid";
  onViewModeChange: (mode: "table" | "grid") => void;
  totalResults: number;
}

export const AssetFilterBar: React.FC<AssetFilterBarProps> = ({
  searchQuery,
  onSearchChange,
  selectedFormat,
  onFormatChange,
  sortBy,
  onSortChange,
  viewMode,
  onViewModeChange,
  totalResults,
}) => {
  const formats = [
    { id: "all", label: "全部格式" },
    { id: "mp4", label: "MP4" },
    { id: "mov", label: "MOV" },
    { id: "mkv", label: "MKV" },
    { id: "webm", label: "WEBM" },
    { id: "flv", label: "FLV" },
  ];

  return (
    <div className="flex flex-wrap items-center justify-between gap-4 p-4 rounded-2xl bg-[#121622] border border-[#1e2433]">
      {/* 搜索框 */}
      <div className="relative min-w-[260px] flex-1 max-w-md">
        <Search className="w-4 h-4 text-slate-400 absolute left-3 top-1/2 -translate-y-1/2" />
        <input
          type="text"
          value={searchQuery}
          onChange={(e) => onSearchChange(e.target.value)}
          placeholder="快速搜索本地视频文件名、关键词..."
          className="w-full bg-[#0a0c10] border border-slate-700/80 rounded-xl pl-9 pr-3 py-2 text-xs text-white placeholder-slate-400 focus:outline-none focus:border-cyan-500 transition-colors"
        />
      </div>

      {/* 格式过滤药丸 */}
      <div className="flex items-center gap-1.5 overflow-x-auto">
        {formats.map((fmt) => (
          <button
            key={fmt.id}
            onClick={() => onFormatChange(fmt.id)}
            className={`px-3 py-1.5 rounded-lg text-xs font-semibold transition-all ${
              selectedFormat === fmt.id
                ? "bg-cyan-950 text-cyan-300 border border-cyan-500/40 shadow-sm"
                : "bg-slate-900/60 text-slate-400 hover:text-slate-200 hover:bg-slate-800/60 border border-slate-800"
            }`}
          >
            {fmt.label}
          </button>
        ))}
      </div>

      {/* 排序与视图切换 */}
      <div className="flex items-center gap-3 ml-auto">
        {/* 排序下拉 */}
        <div className="flex items-center gap-1.5 bg-[#0a0c10] border border-slate-700/80 rounded-xl px-2.5 py-1.5 text-xs text-slate-300">
          <ArrowDownUp className="w-3.5 h-3.5 text-slate-400" />
          <select
            value={sortBy}
            onChange={(e) => onSortChange(e.target.value)}
            className="bg-transparent text-xs text-slate-200 focus:outline-none cursor-pointer"
          >
            <option value="created_desc">最新添加</option>
            <option value="size_desc">文件体积 (大到小)</option>
            <option value="size_asc">文件体积 (小到大)</option>
            <option value="duration_desc">视频时长 (长到短)</option>
            <option value="name_asc">文件名 (A-Z)</option>
          </select>
        </div>

        {/* 表格 / 网格 切换 */}
        <div className="flex items-center bg-[#0a0c10] border border-slate-700/80 rounded-xl p-0.5">
          <button
            onClick={() => onViewModeChange("table")}
            className={`p-1.5 rounded-lg transition-all ${
              viewMode === "table"
                ? "bg-slate-800 text-cyan-400 shadow-sm"
                : "text-slate-400 hover:text-slate-300"
            }`}
            title="表格视图"
          >
            <List className="w-4 h-4" />
          </button>
          <button
            onClick={() => onViewModeChange("grid")}
            className={`p-1.5 rounded-lg transition-all ${
              viewMode === "grid"
                ? "bg-slate-800 text-cyan-400 shadow-sm"
                : "text-slate-400 hover:text-slate-300"
            }`}
            title="网格视图"
          >
            <LayoutGrid className="w-4 h-4" />
          </button>
        </div>

        <span className="text-xs text-slate-400 font-mono hidden md:inline">
          共 <strong className="text-slate-200">{totalResults}</strong> 个视频
        </span>
      </div>
    </div>
  );
};
