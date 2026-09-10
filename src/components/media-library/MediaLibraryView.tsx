import React, { useState, useEffect } from "react";
import { ScanControlBar } from "./ScanControlBar";
import { MetricsDashboard } from "./MetricsDashboard";
import { AssetFilterBar } from "./AssetFilterBar";
import { AssetTableView } from "./AssetTableView";
import { AssetGridCard } from "./AssetGridCard";
import { MediaAssetItem, LibraryMetrics } from "../../types/library";

interface MediaLibraryViewProps {
  onLoadIntoStudio: (asset: MediaAssetItem) => void;
  onUpdateCount?: (count: number) => void;
}

export const MediaLibraryView: React.FC<MediaLibraryViewProps> = ({
  onLoadIntoStudio,
  onUpdateCount,
}) => {
  const [viewMode, setViewMode] = useState<"table" | "grid">("table");
  const [searchQuery, setSearchQuery] = useState("");
  const [selectedFormat, setSelectedFormat] = useState<string>("all");
  const [sortBy, setSortBy] = useState<string>("created_desc");

  const [isScanning, setIsScanning] = useState(false);
  const [scanProgressText, setScanProgressText] = useState("");
  const [totalDiscovered, setTotalDiscovered] = useState(0);

  // 初始指标
  const [metrics, setMetrics] = useState<LibraryMetrics>({
    totalCount: 3,
    totalSizeBytes: 9240000000,
    totalSizeFormatted: "8.61 GB",
    totalDurationSeconds: 7120,
    totalDurationFormatted: "1.98 小时",
    formatStats: [
      { format: "MP4", count: 1, percentage: 55, color: "#00f0ff" },
      { format: "MOV", count: 1, percentage: 25, color: "#a855f7" },
      { format: "MKV", count: 1, percentage: 20, color: "#38bdf8" },
    ],
  });

  // 本地视频资产列表
  const [assets, setAssets] = useState<MediaAssetItem[]>([
    {
      id: 1,
      name: "esports_final_championship_match.mp4",
      fullPath: "/Users/hou/Movies/esports_final_championship_match.mp4",
      fileSize: 5120000000,
      fileSizeFormatted: "4.77 GB",
      duration: 2540,
      durationFormatted: "42:20",
      width: 1920,
      height: 1080,
      resolution: "1080P 60FPS",
      format: "mp4",
      createdAt: Math.floor(Date.now() / 1000) - 86400,
      status: "ready",
      highlightPotentialScore: 0.96,
    },
    {
      id: 2,
      name: "podcast_ai_agents_interview.mov",
      fullPath: "/Users/hou/Movies/podcast_ai_agents_interview.mov",
      fileSize: 2280000000,
      fileSizeFormatted: "2.12 GB",
      duration: 3600,
      durationFormatted: "60:00",
      width: 1920,
      height: 1080,
      resolution: "1080P 30FPS",
      format: "mov",
      createdAt: Math.floor(Date.now() / 1000) - 172800,
      status: "ready",
      highlightPotentialScore: 0.88,
    },
    {
      id: 3,
      name: "action_cam_skiing_extreme.mkv",
      fullPath: "/Users/hou/Downloads/action_cam_skiing_extreme.mkv",
      fileSize: 1840000000,
      fileSizeFormatted: "1.71 GB",
      duration: 980,
      durationFormatted: "16:20",
      width: 2560,
      height: 1440,
      resolution: "2K 120FPS",
      format: "mkv",
      createdAt: Math.floor(Date.now() / 1000) - 21600,
      status: "ready",
      highlightPotentialScore: 0.94,
    },
  ]);

  const formatFileSize = (bytes: number): string => {
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024) return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  };

  const formatDuration = (secs: number): string => {
    const m = Math.floor(secs / 60);
    const s = Math.floor(secs % 60);
    return `${m.toString().padStart(2, "0")}:${s.toString().padStart(2, "0")}`;
  };

  const loadDataFromBackend = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const res: any = await invoke("query_video_assets", {
        page: 1,
        pageSize: 100,
        search: searchQuery || null,
        formatFilter: selectedFormat === "all" ? null : selectedFormat,
        sortBy,
      });

      if (res?.items && Array.isArray(res.items) && res.items.length > 0) {
        const mapped: MediaAssetItem[] = res.items.map((row: any) => ({
          id: row.id,
          name: row.file_name,
          fullPath: row.file_path,
          fileSize: row.file_size,
          fileSizeFormatted: formatFileSize(row.file_size),
          duration: row.duration,
          durationFormatted: row.duration > 0 ? formatDuration(row.duration) : "待探测",
          width: row.width,
          height: row.height,
          resolution: row.width > 0 ? `${row.width}x${row.height}` : "1080P",
          format: row.format,
          createdAt: row.created_at,
          status: row.status,
          highlightPotentialScore: 0.92,
        }));
        setAssets(mapped);
        if (onUpdateCount) onUpdateCount(mapped.length);
      }

      const statsRes: any = await invoke("get_library_metrics");
      if (statsRes) {
        const colors = ["#00f0ff", "#a855f7", "#38bdf8", "#f59e0b", "#10b981"];
        setMetrics({
          totalCount: statsRes.total_count,
          totalSizeBytes: statsRes.total_size_bytes,
          totalSizeFormatted: formatFileSize(statsRes.total_size_bytes),
          totalDurationSeconds: statsRes.total_duration_seconds,
          totalDurationFormatted: `${(statsRes.total_duration_seconds / 3600).toFixed(1)} 小时`,
          formatStats: (statsRes.format_counts || []).map((fc: any, i: number) => ({
            format: fc.format,
            count: fc.count,
            percentage: fc.percentage,
            color: colors[i % colors.length],
          })),
        });
      }
    } catch {
      // 保持当前数据状态
    }
  };

  useEffect(() => {
    loadDataFromBackend();
  }, [searchQuery, selectedFormat, sortBy]);

  // 监听扫描进度
  useEffect(() => {
    let unlistenFn: (() => void) | null = null;
    const setupListener = async () => {
      try {
        const { listen } = await import("@tauri-apps/api/event");
        const unlisten = await listen<any>("scan://progress", (event) => {
          setScanProgressText(`正在扫描: ${event.payload.current_directory}`);
          setTotalDiscovered(event.payload.total_discovered);
          if (event.payload.is_finished) {
            setIsScanning(false);
            loadDataFromBackend();
          }
        });
        unlistenFn = unlisten;
      } catch {
        // Dev fallback
      }
    };
    setupListener();
    return () => {
      if (unlistenFn) unlistenFn();
    };
  }, []);

  const handleScanPreset = async (preset: "movies" | "downloads" | "desktop" | "all") => {
    setIsScanning(true);
    setScanProgressText("启动极速扫描引擎中...");
    setTotalDiscovered(0);

    try {
      const { invoke } = await import("@tauri-apps/api/core");
      let dirs: string[] | null = null;

      if (preset !== "all") {
        const systemDirs: string[] = await invoke("get_system_scan_dirs");
        const matched = systemDirs.filter((d) =>
          d.toLowerCase().includes(preset)
        );
        dirs = matched.length > 0 ? matched : null;
      }

      await invoke("start_video_scan", { customDirs: dirs });
    } catch {
      // Dev mode simulation
      setTimeout(() => {
        setScanProgressText("正在递归扫描 ~/Movies 与 ~/Downloads 目录...");
        setTotalDiscovered(14);
      }, 500);
      setTimeout(() => {
        setScanProgressText("正在批量事务写入 SQLite3 索引中...");
        setTotalDiscovered(28);
      }, 1000);
      setTimeout(() => {
        setIsScanning(false);
        setTotalDiscovered(35);
        // 添加模拟新文件
        setAssets((prev) => [
          {
            id: 4,
            name: "apex_legends_champions_clutch.mp4",
            fullPath: "/Users/hou/Movies/apex_legends_champions_clutch.mp4",
            fileSize: 3400000000,
            fileSizeFormatted: "3.17 GB",
            duration: 1420,
            durationFormatted: "23:40",
            width: 1920,
            height: 1080,
            resolution: "1080P 60FPS",
            format: "mp4",
            createdAt: Math.floor(Date.now() / 1000),
            status: "ready",
            highlightPotentialScore: 0.98,
          },
          ...prev,
        ]);
        setMetrics((prev) => ({
          ...prev,
          totalCount: prev.totalCount + 1,
          totalSizeFormatted: "11.78 GB",
        }));
      }, 1800);
    }
  };

  const handleScanCustom = async () => {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("start_video_scan", { customDirs: null });
    } catch {
      handleScanPreset("movies");
    }
  };

  const handleDeleteAsset = async (id: number | string) => {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("delete_video_asset", { assetId: id });
    } catch {
      // ignore
    }
    setAssets((prev) => prev.filter((a) => a.id !== id));
  };

  // 前端过滤
  const filteredAssets = assets.filter((asset) => {
    const matchesSearch = asset.name.toLowerCase().includes(searchQuery.toLowerCase());
    const matchesFormat = selectedFormat === "all" || asset.format.toLowerCase() === selectedFormat;
    return matchesSearch && matchesFormat;
  });

  return (
    <div className="space-y-6 max-w-[1600px] mx-auto pb-10">
      {/* 1. 顶部扫描控制栏 */}
      <ScanControlBar
        isScanning={isScanning}
        scanProgressText={scanProgressText}
        totalDiscovered={totalDiscovered}
        onScanPreset={handleScanPreset}
        onScanCustom={handleScanCustom}
      />

      {/* 2. 统计卡片区 */}
      <MetricsDashboard metrics={metrics} />

      {/* 3. 过滤检索与视图切换工具条 */}
      <AssetFilterBar
        searchQuery={searchQuery}
        onSearchChange={setSearchQuery}
        selectedFormat={selectedFormat}
        onFormatChange={setSelectedFormat}
        sortBy={sortBy}
        onSortChange={setSortBy}
        viewMode={viewMode}
        onViewModeChange={setViewMode}
        totalResults={filteredAssets.length}
      />

      {/* 4. 资产表格 / 网格 */}
      {viewMode === "table" ? (
        <AssetTableView
          assets={filteredAssets}
          onLoadIntoStudio={onLoadIntoStudio}
          onDeleteAsset={handleDeleteAsset}
        />
      ) : (
        <div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 2xl:grid-cols-4 gap-5">
          {filteredAssets.map((asset) => (
            <AssetGridCard
              key={asset.id}
              asset={asset}
              onLoadIntoStudio={onLoadIntoStudio}
            />
          ))}
        </div>
      )}
    </div>
  );
};
