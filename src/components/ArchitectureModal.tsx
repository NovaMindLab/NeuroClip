import React from "react";
import { X, CheckCircle2, Box, Cpu, FileCode2 } from "lucide-react";

interface ArchitectureModalProps {
  isOpen: boolean;
  onClose: () => void;
}

export const ArchitectureModal: React.FC<ArchitectureModalProps> = ({
  isOpen,
  onClose,
}) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm p-4">
      <div className="bg-[#10141e] border border-[#1e2433] rounded-2xl w-full max-w-2xl max-h-[85vh] overflow-hidden shadow-2xl flex flex-col">
        <div className="px-6 py-4 border-b border-[#1e2433] flex items-center justify-between">
          <div className="flex items-center gap-2">
            <Cpu className="w-5 h-5 text-cyan-400" />
            <h3 className="text-sm font-bold text-slate-100">
              NeuroClip 架构与工程规范 (wiki/readme.txt)
            </h3>
          </div>
          <button
            onClick={onClose}
            className="p-1 rounded-lg hover:bg-slate-800 text-slate-400 hover:text-slate-200 transition-colors"
          >
            <X className="w-5 h-5" />
          </button>
        </div>

        <div className="p-6 overflow-y-auto space-y-4 text-xs leading-relaxed text-slate-300">
          {/* Models Size Matrix */}
          <div className="bg-[#0a0c12] border border-slate-800 rounded-xl p-4">
            <div className="flex items-center gap-2 font-bold text-cyan-300 mb-2">
              <Box className="w-4 h-4" />
              <span>严格的端侧体积红线 (总模型+运行时 &lt; 150MB)</span>
            </div>
            <ul className="space-y-1.5 text-slate-300 pl-2">
              <li className="flex items-center gap-2">
                <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                <span><strong>TransNetV2.onnx</strong> (~15MB): 毫秒级轻量镜头切点识别，防粗暴断句。</span>
              </li>
              <li className="flex items-center gap-2">
                <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                <span><strong>YAMNet.onnx + RMS</strong> (~14MB): 抓取瞬时能量高于均值 2.5 倍与欢呼/大笑/掌声。</span>
              </li>
              <li className="flex items-center gap-2">
                <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                <span><strong>Sherpa-onnx / Piper VITS</strong> (~45MB): 离线合成 16kHz WAV 旁白。</span>
              </li>
              <li className="flex items-center gap-2">
                <CheckCircle2 className="w-3.5 h-3.5 text-emerald-400 shrink-0" />
                <span><strong>FFmpeg CLI 管道</strong>: <code>sidechaincompress</code> 侧链平滑下压 12dB + <code>subtitles</code> 滤镜烧录。</span>
              </li>
            </ul>
          </div>

          {/* Golden Hook & Speech Pace */}
          <div className="bg-[#0a0c12] border border-slate-800 rounded-xl p-4">
            <div className="flex items-center gap-2 font-bold text-purple-300 mb-2">
              <FileCode2 className="w-4 h-4" />
              <span>智能解说词黄金 Hook 与语速对齐</span>
            </div>
            <p className="text-slate-400 mb-2">
              配音语速严格按中文 <strong>4.2 字/秒</strong> 计算，保证前 3 秒黄金悬念 Hook（如“注意看”、“谁能想到”），字数误差控制在 ±3 字之内，输出纯标准 JSON 结构。
            </p>
            <div className="bg-slate-950 p-2.5 rounded-lg font-mono text-[11px] text-cyan-300 border border-slate-800">
              目标字数 = Math.round(duration_seconds * 4.2)
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};
