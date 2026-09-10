use std::path::Path;

pub struct ScanFilter;

impl ScanFilter {
    /// 判定目录是否属于深度剪枝黑名单（直接跳过子树遍历）
    pub fn is_blacklisted_directory(path: &Path) -> bool {
        let full_str = path.to_string_lossy();
        let normalized = full_str.replace('\\', "/");
        let dir_name = normalized.split('/').filter(|s| !s.is_empty()).last().unwrap_or("");

        // 1. 常见代码仓库、构建输出与开发缓存
        if matches!(
            dir_name,
            ".git" | ".svn" | ".hg" | "node_modules" | "target" | "dist" | "build" | ".venv" | "__pycache__" | "vendor" | ".idea" | ".vscode" | ".next"
        ) {
            return true;
        }

        // 2. 系统核心垃圾桶与元数据索引
        if matches!(
            dir_name,
            "$Recycle.Bin" | "System Volume Information" | ".Trash" | ".Spotlight-V100" | ".fseventsd"
        ) || normalized.contains("System Volume Information") || normalized.contains("$Recycle.Bin") {
            return true;
        }

        let full_str = path.to_string_lossy();

        // 3. macOS 深度防卡死系统路径
        if full_str.contains("/Library/Caches")
            || full_str.contains("/Library/Application Support")
            || full_str.starts_with("/private/var")
            || full_str.starts_with("/System")
            || full_str.starts_with("/dev")
            || full_str.starts_with("/proc")
        {
            return true;
        }

        // 4. Windows 深度防卡死路径
        if full_str.contains(r"\AppData\Local\Temp") || full_str.contains(r"\AppData\Local\Packages") {
            return true;
        }

        // 5. 默认跳过非根节点的隐藏文件夹（以 . 开头）
        if dir_name.starts_with('.') && dir_name != "." && dir_name != ".." {
            return true;
        }

        false
    }

    /// 校验扩展名是否为白名单视频格式
    pub fn is_supported_video_format(path: &Path) -> bool {
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => return false,
        };

        matches!(
            ext.as_str(),
            "mp4" | "mkv" | "mov" | "avi" | "flv" | "webm" | "ts" | "wmv" | "m4v"
        )
    }
}
