import React from "react";
import { NeuroClipLogo } from "../NeuroClipLogo";
import {
  LayoutDashboard,
  Film,
  Eye,
  Settings,
  ChevronLeft,
  ChevronRight,
  Zap,
} from "lucide-react";
import { AppNavTab } from "../../types/library";

interface SidebarProps {
  isCollapsed: boolean;
  onToggleCollapse: () => void;
  activeTab: AppNavTab;
  onSelectTab: (tab: AppNavTab) => void;
  hwEncoderName: string;
  isWatcherActive: boolean;
  libraryCount: number;
}

export const Sidebar: React.FC<SidebarProps> = ({
  isCollapsed,
  onToggleCollapse,
  activeTab,
  onSelectTab,
  hwEncoderName,
  isWatcherActive,
  libraryCount,
}) => {
  const navItems: { id: AppNavTab; label: string; icon: React.ReactNode; badge?: string }[] = [
    { id: "studio", label: "高光剪辑工作台", icon: <LayoutDashboard className="w-5 h-5" /> },
    { id: "library", label: "PC 本地媒体库", icon: <Film className="w-5 h-5" />, badge: libraryCount > 0 ? `${libraryCount}` : undefined },
    { id: "watcher", label: "无人值守监听", icon: <Eye className="w-5 h-5" />, badge: isWatcherActive ? "ON" : undefined },
    { id: "settings", label: "系统与硬件设置", icon: <Settings className="w-5 h-5" /> },
  ];

  return (
    <aside
      className={`h-screen border-r border-[#1e2433] bg-[#0d1118]/95 backdrop-blur-xl flex flex-col justify-between transition-all duration-300 z-30 shrink-0 ${
        isCollapsed ? "w-[72px]" : "w-[240px]"
      }`}
    >
      {/* 顶部 Logo 与品牌 */}
      <div className="p-4 border-b border-[#1e2433]/80 flex items-center justify-between">
        <div className="overflow-hidden flex items-center">
          <NeuroClipLogo size={36} showText={!isCollapsed} />
        </div>
      </div>

      {/* 中部核心导航菜单 */}
      <nav className="flex-1 py-4 px-3 space-y-1.5 overflow-y-auto">
        {navItems.map((item) => {
          const isActive = activeTab === item.id;
          return (
            <button
              key={item.id}
              onClick={() => onSelectTab(item.id)}
              title={isCollapsed ? item.label : undefined}
              className={`w-full flex items-center gap-3 px-3 py-2.5 rounded-xl text-xs font-semibold transition-all relative group ${
                isActive
                  ? "bg-gradient-to-r from-cyan-950/80 to-blue-950/50 text-cyan-300 border border-cyan-500/40 shadow-[0_0_15px_rgba(0,240,255,0.15)]"
                  : "text-slate-400 hover:text-slate-200 hover:bg-slate-900/60 border border-transparent"
              }`}
            >
              <div className={`${isActive ? "text-cyan-400" : "text-slate-400 group-hover:text-cyan-400"}`}>
                {item.icon}
              </div>

              {!isCollapsed && (
                <span className="flex-1 text-left tracking-wide">{item.label}</span>
              )}

              {/* 徽标 / 状态指示 */}
              {item.badge && !isCollapsed && (
                <span
                  className={`text-[10px] px-1.5 py-0.5 rounded font-mono font-bold ${
                    item.id === "watcher" && isWatcherActive
                      ? "bg-emerald-950 text-emerald-300 border border-emerald-500/40 animate-pulse"
                      : "bg-slate-800 text-cyan-300 border border-slate-700"
                  }`}
                >
                  {item.badge}
                </span>
              )}

              {/* 折叠模式下小圆点 */}
              {item.badge && isCollapsed && (
                <span className="absolute top-2 right-2 w-2 h-2 rounded-full bg-cyan-400 ring-2 ring-[#0d1118]" />
              )}
            </button>
          );
        })}
      </nav>

      {/* 底部硬件加速胶囊与折叠开关 */}
      <div className="p-3 border-t border-[#1e2433]/80 space-y-3">
        {!isCollapsed ? (
          <div className="p-2.5 rounded-xl bg-[#121620] border border-amber-500/20 text-slate-300">
            <div className="flex items-center gap-2 mb-1 text-[11px] font-bold text-amber-300">
              <Zap className="w-3.5 h-3.5 text-amber-400" />
              <span>硬件加速引擎</span>
            </div>
            <div className="text-[10px] text-slate-400 truncate" title={hwEncoderName}>
              {hwEncoderName}
            </div>
          </div>
        ) : (
          <div className="flex justify-center" title={`硬件加速: ${hwEncoderName}`}>
            <div className="w-8 h-8 rounded-lg bg-amber-950/50 border border-amber-500/30 flex items-center justify-center text-amber-400">
              <Zap className="w-4 h-4" />
            </div>
          </div>
        )}

        <div className="flex items-center justify-between text-slate-400 text-[11px] pt-1">
          {!isCollapsed && (
            <span className="font-mono text-[10px] text-slate-400">NeuroClip v0.3.0</span>
          )}
          <button
            onClick={onToggleCollapse}
            className="p-1.5 rounded-lg hover:bg-slate-800 hover:text-slate-200 transition-colors ml-auto"
            title={isCollapsed ? "展开侧边栏" : "收起侧边栏"}
          >
            {isCollapsed ? <ChevronRight className="w-4 h-4" /> : <ChevronLeft className="w-4 h-4" />}
          </button>
        </div>
      </div>
    </aside>
  );
};
