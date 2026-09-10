import React, { useState } from "react";
import { Sidebar } from "./Sidebar";
import { WorkspaceHeader } from "./WorkspaceHeader";
import { AppNavTab } from "../../types/library";

interface AppLayoutProps {
  activeTab: AppNavTab;
  onSelectTab: (tab: AppNavTab) => void;
  hwEncoderName: string;
  isWatcherActive: boolean;
  onToggleWatcher: () => void;
  ffmpegReady: boolean;
  onOpenInfoModal: () => void;
  libraryCount: number;
  children: React.ReactNode;
}

export const AppLayout: React.FC<AppLayoutProps> = ({
  activeTab,
  onSelectTab,
  hwEncoderName,
  isWatcherActive,
  onToggleWatcher,
  ffmpegReady,
  onOpenInfoModal,
  libraryCount,
  children,
}) => {
  const [isSidebarCollapsed, setIsSidebarCollapsed] = useState(false);

  return (
    <div className="flex h-screen w-screen bg-[#0a0c10] text-slate-100 overflow-hidden font-sans select-none">
      {/* 1. 左侧固定侧边栏 */}
      <Sidebar
        isCollapsed={isSidebarCollapsed}
        onToggleCollapse={() => setIsSidebarCollapsed(!isSidebarCollapsed)}
        activeTab={activeTab}
        onSelectTab={onSelectTab}
        hwEncoderName={hwEncoderName}
        isWatcherActive={isWatcherActive}
        libraryCount={libraryCount}
      />

      {/* 2. 右侧主工作区容器 */}
      <div className="flex-1 flex flex-col min-w-0 overflow-hidden bg-[#0a0c10]">
        <WorkspaceHeader
          activeTab={activeTab}
          ffmpegReady={ffmpegReady}
          isWatcherActive={isWatcherActive}
          onToggleWatcher={onToggleWatcher}
          onOpenInfoModal={onOpenInfoModal}
        />

        {/* 动态视口滚动容器 */}
        <main className="flex-1 overflow-y-auto relative p-6">
          {children}
        </main>
      </div>
    </div>
  );
};
