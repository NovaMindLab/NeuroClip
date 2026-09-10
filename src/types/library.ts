export type AppNavTab = "studio" | "library" | "watcher" | "settings";

export interface MediaAssetItem {
  id: number | string;
  name: string;
  fullPath: string;
  fileSize: number;
  fileSizeFormatted: string;
  duration: number;
  durationFormatted: string;
  width: number;
  height: number;
  resolution: string;
  format: string;
  createdAt: number;
  status: string;
  highlightPotentialScore?: number;
}

export interface FormatStat {
  format: string;
  count: number;
  percentage: number;
  color: string;
}

export interface LibraryMetrics {
  totalCount: number;
  totalSizeBytes: number;
  totalSizeFormatted: string;
  totalDurationSeconds: number;
  totalDurationFormatted: string;
  formatStats: FormatStat[];
}
