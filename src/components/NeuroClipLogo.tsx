import React from "react";

interface LogoProps {
  size?: number;
  showText?: boolean;
  className?: string;
}

export const NeuroClipLogo: React.FC<LogoProps> = ({
  size = 40,
  showText = true,
  className = "",
}) => {
  return (
    <div className={`flex items-center gap-3 ${className}`}>
      <div
        className="relative flex items-center justify-center rounded-2xl bg-gradient-to-br from-slate-900 via-[#10141e] to-slate-950 p-1 border border-cyan-500/30 shadow-lg shadow-cyan-500/10 neuro-logo-glow"
        style={{ width: size, height: size }}
      >
        <svg
          viewBox="0 0 100 100"
          className="w-full h-full"
          fill="none"
          xmlns="http://www.w3.org/2000/svg"
        >
          <defs>
            <linearGradient id="cyanPurpleGrad" x1="0%" y1="0%" x2="100%" y2="100%">
              <stop offset="0%" stopColor="#00f0ff" />
              <stop offset="50%" stopColor="#3b82f6" />
              <stop offset="100%" stopColor="#a855f7" />
            </linearGradient>
            <linearGradient id="neonCyan" x1="0%" y1="100%" x2="100%" y2="0%">
              <stop offset="0%" stopColor="#06b6d4" />
              <stop offset="100%" stopColor="#22d3ee" />
            </linearGradient>
            <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">
              <feGaussianBlur stdDeviation="3" result="blur" />
              <feComposite in="SourceGraphic" in2="blur" operator="over" />
            </filter>
          </defs>

          {/* 外圈胶片与神经环 */}
          <circle
            cx="50"
            cy="50"
            r="42"
            stroke="url(#cyanPurpleGrad)"
            strokeWidth="3.5"
            strokeDasharray="4 6 12 4"
            className="opacity-90"
          />

          {/* 突触核心节点 */}
          <circle cx="28" cy="34" r="4.5" fill="#00f0ff" filter="url(#glow)" />
          <circle cx="72" cy="66" r="4.5" fill="#c084fc" filter="url(#glow)" />
          <circle cx="68" cy="32" r="3" fill="#38bdf8" />
          <circle cx="32" cy="68" r="3" fill="#a855f7" />

          {/* 突触神经连接曲线 */}
          <path
            d="M28 34 C 40 28, 48 44, 68 32"
            stroke="url(#neonCyan)"
            strokeWidth="2.5"
            strokeLinecap="round"
            className="opacity-75"
          />
          <path
            d="M32 68 C 48 56, 58 72, 72 66"
            stroke="#c084fc"
            strokeWidth="2.5"
            strokeLinecap="round"
            className="opacity-75"
          />

          {/* 胶片剪刀剪辑核心象征 */}
          <path
            d="M38 62 L 62 38"
            stroke="#ffffff"
            strokeWidth="3.5"
            strokeLinecap="round"
          />
          <path
            d="M38 38 L 62 62"
            stroke="url(#cyanPurpleGrad)"
            strokeWidth="3.5"
            strokeLinecap="round"
          />
          <circle cx="50" cy="50" r="3.5" fill="#00f0ff" />
        </svg>
      </div>

      {showText && (
        <div className="flex flex-col">
          <div className="flex items-center gap-1.5">
            <span className="text-lg font-extrabold tracking-tight bg-clip-text text-transparent bg-gradient-to-r from-cyan-400 via-sky-300 to-purple-400">
              NeuroClip
            </span>
            <span className="px-1.5 py-0.5 text-[10px] font-semibold bg-cyan-950/80 text-cyan-300 rounded border border-cyan-500/30">
              AI v2.0
            </span>
          </div>
          <span className="text-[11px] text-slate-400 tracking-wide">
            长视频高光切片与智能解说系统
          </span>
        </div>
      )}
    </div>
  );
};
