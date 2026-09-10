import React, { useState } from "react";
import { Sparkles } from "lucide-react";
import { MediaAssetItem } from "../../types/library";

interface QuickClipActionBtnProps {
  asset: MediaAssetItem;
  onLoad: (asset: MediaAssetItem) => void;
  compact?: boolean;
}

export const QuickClipActionBtn: React.FC<QuickClipActionBtnProps> = ({
  asset,
  onLoad,
  compact = false,
}) => {
  const [isHovered, setIsHovered] = useState(false);

  return (
    <button
      onClick={(e) => {
        e.stopPropagation();
        onLoad(asset);
      }}
      onMouseEnter={() => setIsHovered(true)}
      onMouseLeave={() => setIsHovered(false)}
      className={`relative inline-flex items-center justify-center gap-1.5 font-bold transition-all rounded-xl overflow-hidden group shadow-lg active:scale-95 ${
        compact ? "px-3 py-1.5 text-xs" : "px-4 py-2 text-xs"
      } bg-gradient-to-r from-cyan-500 via-blue-600 to-purple-600 text-white shadow-cyan-500/20 hover:shadow-cyan-500/40 hover:brightness-110`}
      title="一键将视频载入高光工作台开始自动化切片"
    >
      <span className="text-sm">🚀</span>
      <span>载入工作台切片</span>
      <Sparkles
        className={`w-3.5 h-3.5 transition-transform duration-300 ${
          isHovered ? "rotate-45 scale-125 text-amber-300" : "text-cyan-200"
        }`}
      />
    </button>
  );
};
