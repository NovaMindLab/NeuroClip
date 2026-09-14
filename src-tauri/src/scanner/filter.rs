use std::path::Path;

pub struct ScanFilter;

impl ScanFilter {
    /// 判定目录是否属于深度剪枝黑名单（直接跳过子树遍历）
    pub fn is_blacklisted_directory(path: &Path) -> bool {
        let full_str = path.to_string_lossy();
        let normalized = full_str.replace('\\', "/");
        let dir_name = normalized.split('/').filter(|s| !s.is_empty()).last().unwrap_or("");

        // 1. 常见代码仓库、构建输出、包管理器与开发缓存
        if matches!(
            dir_name,
            ".git" | ".svn" | ".hg" | "node_modules" | "target" | "dist" | "build" | "out" | "bin" | "obj"
                | ".venv" | "venv" | "env" | "__pycache__" | "vendor" | "bower_components"
                | ".idea" | ".vscode" | ".fleet"
                | ".next" | ".nuxt" | ".turbo" | ".svelte-kit" | ".output"
                | ".cargo" | ".rustup" | ".gradle" | ".m2" | ".pnpm-store" | ".yarn" | "coverage"
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

    /// 校验文件是否为受支持的真实视频格式（对 .ts 等双重用途扩展名实施严格二进制特征校验）
    pub fn is_supported_video_format(path: &Path) -> bool {
        let ext = match path.extension().and_then(|e| e.to_str()) {
            Some(e) => e.to_lowercase(),
            None => return false,
        };

        match ext.as_str() {
            // 无歧义的标准视频格式
            "mp4" | "mkv" | "mov" | "avi" | "flv" | "webm" | "wmv" | "m4v" => true,
            // 容易与 TypeScript 源码严重混淆的格式，必须验证文件二进制头与 MPEG-TS 同步字节
            "ts" | "m2ts" | "mts" => Self::is_valid_mpeg_ts(path),
            _ => false,
        }
    }

    /// 严格验证是否为真实的 MPEG-2 Transport Stream 视频文件，彻底排除 TypeScript 代码文件
    pub fn is_valid_mpeg_ts(path: &Path) -> bool {
        let file_name = match path.file_name().and_then(|n| n.to_str()) {
            Some(n) => n.to_lowercase(),
            None => return false,
        };

        // 1. 文件名黑名单过滤：排除 TypeScript 声明及各类典型源码模式
        if file_name.starts_with('.')
            || file_name.ends_with(".d.ts")
            || file_name.ends_with(".test.ts")
            || file_name.ends_with(".spec.ts")
            || file_name.ends_with(".config.ts")
            || file_name.ends_with(".routes.ts")
            || file_name.ends_with(".router.ts")
            || file_name.ends_with(".service.ts")
            || file_name.ends_with(".controller.ts")
            || file_name.ends_with(".module.ts")
            || file_name.ends_with(".component.ts")
            || file_name.ends_with(".types.ts")
            || file_name.ends_with(".model.ts")
        {
            return false;
        }

        // 2. 文件元数据检查：排除不存在或过小的文件
        let meta = match std::fs::metadata(path) {
            Ok(m) => m,
            Err(_) => return false,
        };

        let file_size = meta.len();
        // 小于 188 字节绝对不可能是任何合法的 MPEG-TS 视频包
        if file_size < 188 {
            return false;
        }

        // 3. 读取头部 564 字节（3 个标准 188 字节 TS 包）
        use std::io::Read;
        let mut file = match std::fs::File::open(path) {
            Ok(f) => f,
            Err(_) => return false,
        };

        let mut buffer = [0u8; 564];
        let bytes_read = match file.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return false,
        };

        if bytes_read < 188 {
            return false;
        }

        // 4. 文本源码启发式过滤：如果头部为可打印文本代码特征，直接判定为源码排除
        let sample_len = bytes_read.min(256);
        if let Ok(text_sample) = std::str::from_utf8(&buffer[..sample_len]) {
            let lower = text_sample.to_lowercase();
            if lower.contains("import ")
                || lower.contains("export ")
                || lower.contains("const ")
                || lower.contains("function ")
                || lower.contains("interface ")
                || lower.contains("class ")
                || lower.contains("type ")
                || lower.contains("let ")
                || lower.contains("var ")
                || lower.contains("enum ")
                || lower.contains("declare ")
                || lower.contains("//")
                || lower.contains("/*")
                || lower.contains("from \"")
                || lower.contains("from '")
                || lower.contains("<div")
                || lower.contains("return ")
            {
                return false;
            }
        }

        // 5. ISO/IEC 13818-1 MPEG-TS 同步字节检测 (Sync Byte 0x47 / 'G')
        // 模式 A: 标准 188 字节包 (连续包 offset 0, 188, 376 均为 0x47)
        if buffer[0] == 0x47 {
            if bytes_read >= 376 {
                if buffer[188] == 0x47 && (bytes_read < 564 || buffer[376] == 0x47) {
                    return true;
                }
            } else if bytes_read >= 188 {
                return true;
            }
        }

        // 模式 B: 192 字节包 (M2TS / BDAV, 步长 192, 0x47 在 offset 0 或 offset 4)
        if bytes_read >= 384 {
            if buffer[0] == 0x47 && buffer[192] == 0x47 {
                return true;
            }
            if bytes_read >= 388 && buffer[4] == 0x47 && buffer[196] == 0x47 {
                return true;
            }
        }

        // 模式 C: 204 字节包 (DVB Reed-Solomon FEC 封装, 步长 204)
        if bytes_read >= 408 && buffer[0] == 0x47 && buffer[204] == 0x47 {
            return true;
        }

        false
    }
}
