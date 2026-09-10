import React from "react";
import { Video, HardDrive, Clock, PieChart } from "lucide-react";
import { LibraryMetrics } from "../../types/library";

interface MetricsDashboardProps {
  metrics: LibraryMetrics;
}

export const MetricsDashboard: React.FC<MetricsDashboardProps> = ({ metrics }) => {
  return (
    <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
      {/* 1. 总视频数 */}
      <div className="p-4 rounded-2xl bg-[#121622] border border-[#1e2433] flex items-center justify-between shadow-lg">
        <div>
          <span className="text-xs text-slate-400 font-medium">已入库视频总数</span>
          <div className="text-2xl font-black text-white mt-1 font-mono tracking-tight flex items-baseline gap-1.5">
            <span>{metrics.totalCount}</span>
            <span className="text-xs font-normal text-slate-400">部</span>
          </div>
          <span className="text-[11px] text-cyan-400 mt-0.5 inline-block">SQLite3 索引就绪</span>
        </div>
        <div className="w-10 h-10 rounded-xl bg-cyan-950/60 border border-cyan-500/30 flex items-center justify-center text-cyan-400">
          <Video className="w-5 h-5" />
        </div>
      </div>

      {/* 2. 占用磁盘总容量 */}
      <div className="p-4 rounded-2xl bg-[#121622] border border-[#1e2433] flex items-center justify-between shadow-lg">
        <div>
          <span className="text-xs text-slate-400 font-medium">占用磁盘总容量</span>
          <div className="text-2xl font-black text-white mt-1 font-mono tracking-tight flex items-baseline gap-1.5">
            <span>{metrics.totalSizeFormatted}</span>
          </div>
          <span className="text-[11px] text-purple-400 mt-0.5 inline-block">无压缩直存原画</span>
        </div>
        <div className="w-10 h-10 rounded-xl bg-purple-950/60 border border-purple-500/30 flex items-center justify-center text-purple-400">
          <HardDrive className="w-5 h-5" />
        </div>
      </div>

      {/* 3. 累计时长 */}
      <div className="p-4 rounded-2xl bg-[#121622] border border-[#1e2433] flex items-center justify-between shadow-lg">
        <div>
          <span className="text-xs text-slate-400 font-medium">累计视频时长</span>
          <div className="text-2xl font-black text-white mt-1 font-mono tracking-tight flex items-baseline gap-1.5">
            <span>{metrics.totalDurationFormatted}</span>
          </div>
          <span className="text-[11px] text-emerald-400 mt-0.5 inline-block">可提纯高光矿藏</span>
        </div>
        <div className="w-10 h-10 rounded-xl bg-emerald-950/60 border border-emerald-500/30 flex items-center justify-center text-emerald-400">
          <Clock className="w-5 h-5" />
        </div>
      </div>

      {/* 4. 格式分布 */}
      <div className="p-4 rounded-2xl bg-[#121622] border border-[#1e2433] flex flex-col justify-between shadow-lg">
        <div className="flex items-center justify-between mb-2">
          <span className="text-xs text-slate-400 font-medium">媒体格式分布</span>
          <PieChart className="w-4 h-4 text-amber-400" />
        </div>

        {metrics.formatStats.length > 0 ? (
          <div className="space-y-1.5">
            <div className="flex h-2 rounded-full overflow-hidden bg-slate-800">
              {metrics.formatStats.map((st, i) => (
                <div
                  key={i}
                  style={{ width: `${Math.max(st.percentage, 5)}%`, backgroundColor: st.color }}
                  title={`${st.format}: ${st.count}个 (${st.percentage.toFixed(1)}%)`}
                />
              ))}
            </div>
            <div className="flex items-center gap-2 overflow-x-auto text-[10px] text-slate-400 font-mono">
              {metrics.formatStats.slice(0, 3).map((st, i) => (
                <span key={i} className="flex items-center gap-1 shrink-0">
                  <span className="w-2 h-2 rounded-full" style={{ backgroundColor: st.color }} />
                  <span>{st.format} {st.percentage.toFixed(0)}%</span>
                </span>
              ))}
            </div>
          </div>
        ) : (
          <div className="text-xs text-slate-400">暂无入库数据</div>
        )}
      </div>
    </div>
  );
};
