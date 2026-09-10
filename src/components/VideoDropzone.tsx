import React, { useState } from "react";
import { UploadCloud, FileVideo, Play, CheckCircle2, Smartphone, Monitor } from "lucide-react";

interface VideoDropzoneProps {
  onVideoSelected: (filePath: string, fileName: string) => void;
  selectedVideoPath: string | null;
  selectedVideoName: string | null;
  isProcessing: boolean;
  onStartProcess: () => void;
  isVertical: boolean;
  onToggleAspect: (val: boolean) => void;
}

export const VideoDropzone: React.FC<VideoDropzoneProps> = ({
  onVideoSelected,
  selectedVideoPath,
  selectedVideoName,
  isProcessing,
  onStartProcess,
  isVertical,
  onToggleAspect,
}) => {
  const [isDragOver, setIsDragOver] = useState(false);

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(true);
  };

  const handleDragLeave = () => {
    setIsDragOver(false);
  };

  const handleDrop = (e: React.DragEvent) => {
    e.preventDefault();
    setIsDragOver(false);
    if (e.dataTransfer.files && e.dataTransfer.files.length > 0) {
      const file = e.dataTransfer.files[0];
      const fakePath = (file as any).path || `/videos/${file.name}`;
      onVideoSelected(fakePath, file.name);
    }
  };

  const handleDemoSelect = () => {
    onVideoSelected(
      "/samples/esports_final_championship_match.mp4",
      "esports_final_championship_match.mp4"
    );
  };

  return (
    <div className="bg-[#10141e]/70 border border-[#1e2433] rounded-2xl p-6 shadow-xl relative overflow-hidden">
      <div className="flex flex-col lg:flex-row items-center justify-between gap-6">
        {/* Dropzone area */}
        <div
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onDrop={handleDrop}
          onClick={handleDemoSelect}
          className={`flex-1 w-full border-2 border-dashed rounded-xl p-6 flex flex-col items-center justify-center text-center cursor-pointer transition-all duration-200 ${
            isDragOver
              ? "border-cyan-400 bg-cyan-950/20"
              : selectedVideoPath
              ? "border-cyan-500/40 bg-slate-900/60"
              : "border-slate-700/80 hover:border-slate-600 bg-slate-900/30"
          }`}
        >
          {selectedVideoPath ? (
            <div className="flex items-center gap-4">
              <div className="w-12 h-12 rounded-xl bg-cyan-500/10 border border-cyan-500/30 flex items-center justify-center text-cyan-400">
                <FileVideo className="w-6 h-6" />
              </div>
              <div className="text-left">
                <div className="flex items-center gap-2">
                  <p className="font-semibold text-sm text-slate-100">{selectedVideoName}</p>
                  <CheckCircle2 className="w-4 h-4 text-cyan-400" />
                </div>
                <p className="text-xs text-slate-400 mt-0.5 truncate max-w-md">{selectedVideoPath}</p>
                <div className="flex items-center gap-3 mt-1.5 text-[11px] text-slate-400">
                  <span>时长: 45 分钟</span>
                  <span>•</span>
                  <span>1080p 60fps</span>
                  <span>•</span>
                  <span>单/双声道自动重采样</span>
                </div>
              </div>
            </div>
          ) : (
            <>
              <div className="w-12 h-12 rounded-xl bg-gradient-to-tr from-cyan-500/10 to-purple-500/10 border border-cyan-500/20 flex items-center justify-center text-cyan-300 mb-2">
                <UploadCloud className="w-6 h-6" />
              </div>
              <p className="text-sm font-semibold text-slate-200">
                拖拽长视频文件至此处，或点击加载测试电竞比赛/播客录像
              </p>
              <p className="text-xs text-slate-400 mt-0.5">
                支持 MP4, MKV, MOV, FLV, TS 格式 (纯短视频成片输出)
              </p>
            </>
          )}
        </div>

        {/* Aspect Ratio & Action Button Controls */}
        <div className="w-full lg:w-auto flex flex-col gap-3">
          {/* 9:16 vs 16:9 Switcher */}
          <div className="flex items-center justify-between p-1.5 rounded-xl bg-slate-900/90 border border-slate-800">
            <button
              onClick={() => onToggleAspect(true)}
              className={`flex-1 flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all ${
                isVertical
                  ? "bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/20"
                  : "text-slate-400 hover:text-slate-200"
              }`}
            >
              <Smartphone className="w-3.5 h-3.5" />
              <span>9:16 竖屏模糊</span>
            </button>

            <button
              onClick={() => onToggleAspect(false)}
              className={`flex-1 flex items-center justify-center gap-1.5 px-3 py-1.5 rounded-lg text-xs font-semibold transition-all ${
                !isVertical
                  ? "bg-cyan-500 text-slate-950 shadow-md shadow-cyan-500/20"
                  : "text-slate-400 hover:text-slate-200"
              }`}
            >
              <Monitor className="w-3.5 h-3.5" />
              <span>16:9 横屏原画</span>
            </button>
          </div>

          <button
            onClick={onStartProcess}
            disabled={!selectedVideoPath || isProcessing}
            className={`px-7 py-3 rounded-xl font-semibold text-sm flex items-center justify-center gap-2.5 shadow-lg transition-all ${
              !selectedVideoPath || isProcessing
                ? "bg-slate-800/80 text-slate-500 border border-slate-700 cursor-not-allowed"
                : "bg-gradient-to-r from-cyan-500 via-sky-500 to-purple-600 hover:from-cyan-400 hover:to-purple-500 text-white border border-cyan-300/40 shadow-cyan-500/25 cursor-pointer active:scale-95"
            }`}
          >
            <Play className={`w-4 h-4 ${isProcessing ? "animate-spin" : "fill-current"}`} />
            <span>{isProcessing ? "AI 全流程短视频切片中..." : "启动 AI 智能高光与解说切片"}</span>
          </button>

          <button
            onClick={handleDemoSelect}
            className="text-xs text-slate-400 hover:text-cyan-300 transition-colors text-center py-0.5"
          >
            载入预设高能比赛长视频样本
          </button>
        </div>
      </div>
    </div>
  );
};
