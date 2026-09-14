use std::path::Path;
use neuroclip_lib::db::{DatabaseManager, VideoAssetRow};
use neuroclip_lib::scanner::ScanFilter;

#[test]
fn test_db_initialization_and_schema() {
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("test_assets.db");

    let mgr = DatabaseManager::new(&db_path).expect("初始化测试数据库失败");
    let stats = mgr.get_library_stats().expect("获取统计失败");
    assert_eq!(stats.total_count, 0);
    assert_eq!(stats.total_size_bytes, 0);
}

#[test]
fn test_bulk_upsert_and_conflict() {
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("test_upsert.db");
    let mgr = DatabaseManager::new(&db_path).unwrap();

    let items = vec![
        VideoAssetRow {
            id: 0,
            file_path: "/test/videos/match1.mp4".to_string(),
            file_name: "match1.mp4".to_string(),
            file_size: 1048576,
            duration: 120.5,
            width: 1920,
            height: 1080,
            format: "mp4".to_string(),
            created_at: 1700000000,
            modified_at: 1700000000,
            scanned_at: 1700000000,
            status: "ready".to_string(),
        },
        VideoAssetRow {
            id: 0,
            file_path: "/test/videos/highlight2.mov".to_string(),
            file_name: "highlight2.mov".to_string(),
            file_size: 2097152,
            duration: 60.0,
            width: 3840,
            height: 2160,
            format: "mov".to_string(),
            created_at: 1700001000,
            modified_at: 1700001000,
            scanned_at: 1700001000,
            status: "ready".to_string(),
        },
    ];

    let inserted = mgr.bulk_upsert_assets(&items).unwrap();
    assert_eq!(inserted, 2);

    let (total, list) = mgr.query_assets(1, 10, None, None, None).unwrap();
    assert_eq!(total, 2);
    assert_eq!(list.len(), 2);

    // 测试冲突 Upsert：同一路径修改了 modified_at 与 file_size
    let updated_items = vec![
        VideoAssetRow {
            id: 0,
            file_path: "/test/videos/match1.mp4".to_string(),
            file_name: "match1.mp4".to_string(),
            file_size: 1500000,
            duration: 120.5,
            width: 1920,
            height: 1080,
            format: "mp4".to_string(),
            created_at: 1700000000,
            modified_at: 1700002000,
            scanned_at: 1700002000,
            status: "discovered".to_string(),
        },
    ];

    let upserted = mgr.bulk_upsert_assets(&updated_items).unwrap();
    assert_eq!(upserted, 1);

    let (total_after, list_after) = mgr.query_assets(1, 10, None, None, None).unwrap();
    assert_eq!(total_after, 2); // 总记录数仍为 2，未产生重复记录
    let updated_record = list_after.iter().find(|x| x.file_name == "match1.mp4").unwrap();
    assert_eq!(updated_record.file_size, 1500000);
    assert_eq!(updated_record.modified_at, 1700002000);
}

#[test]
fn test_query_filter_and_search() {
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("test_filter.db");
    let mgr = DatabaseManager::new(&db_path).unwrap();

    let items = vec![
        VideoAssetRow {
            id: 0,
            file_path: "/v/csgo_major_final.mp4".to_string(),
            file_name: "csgo_major_final.mp4".to_string(),
            file_size: 5000000,
            duration: 1800.0,
            width: 1920,
            height: 1080,
            format: "mp4".to_string(),
            created_at: 100,
            modified_at: 100,
            scanned_at: 100,
            status: "ready".to_string(),
        },
        VideoAssetRow {
            id: 0,
            file_path: "/v/dota2_ti_championship.mkv".to_string(),
            file_name: "dota2_ti_championship.mkv".to_string(),
            file_size: 8000000,
            duration: 3600.0,
            width: 1920,
            height: 1080,
            format: "mkv".to_string(),
            created_at: 200,
            modified_at: 200,
            scanned_at: 200,
            status: "ready".to_string(),
        },
        VideoAssetRow {
            id: 0,
            file_path: "/v/vlog_tokyo_trip.mov".to_string(),
            file_name: "vlog_tokyo_trip.mov".to_string(),
            file_size: 3000000,
            duration: 300.0,
            width: 3840,
            height: 2160,
            format: "mov".to_string(),
            created_at: 300,
            modified_at: 300,
            scanned_at: 300,
            status: "ready".to_string(),
        },
    ];

    mgr.bulk_upsert_assets(&items).unwrap();

    // 搜索关键词 "csgo"
    let (cnt_search, list_search) = mgr.query_assets(1, 10, Some("csgo"), None, None).unwrap();
    assert_eq!(cnt_search, 1);
    assert_eq!(list_search[0].file_name, "csgo_major_final.mp4");

    // 格式过滤 "mkv"
    let (cnt_mkv, list_mkv) = mgr.query_assets(1, 10, None, Some("mkv"), None).unwrap();
    assert_eq!(cnt_mkv, 1);
    assert_eq!(list_mkv[0].file_name, "dota2_ti_championship.mkv");

    // 统计数据
    let stats = mgr.get_library_stats().unwrap();
    assert_eq!(stats.total_count, 3);
    assert_eq!(stats.total_size_bytes, 16000000);
    assert_eq!(stats.format_counts.len(), 3);
}

#[test]
fn test_scanner_filter_pruning_and_whitelist() {
    // 剪枝测试
    assert!(ScanFilter::is_blacklisted_directory(Path::new("/Users/dev/project/.git")));
    assert!(ScanFilter::is_blacklisted_directory(Path::new("/Users/dev/project/node_modules")));
    assert!(ScanFilter::is_blacklisted_directory(Path::new("/Users/dev/project/target")));
    assert!(ScanFilter::is_blacklisted_directory(Path::new("/Users/user/Library/Caches")));
    assert!(ScanFilter::is_blacklisted_directory(Path::new("/Users/user/.Trash")));
    assert!(ScanFilter::is_blacklisted_directory(Path::new("C:\\Windows\\System Volume Information")));

    // 正常目录不应被剪枝
    assert!(!ScanFilter::is_blacklisted_directory(Path::new("/Users/user/Movies")));
    assert!(!ScanFilter::is_blacklisted_directory(Path::new("/Users/user/Downloads/Videos")));

    // 格式白名单测试
    assert!(ScanFilter::is_supported_video_format(Path::new("game.mp4")));
    assert!(ScanFilter::is_supported_video_format(Path::new("movie.mkv")));
    assert!(ScanFilter::is_supported_video_format(Path::new("clip.MOV")));
    assert!(ScanFilter::is_supported_video_format(Path::new("stream.flv")));
    assert!(ScanFilter::is_supported_video_format(Path::new("raw.webm")));
    assert!(!ScanFilter::is_supported_video_format(Path::new("photo.jpg")));
    assert!(!ScanFilter::is_supported_video_format(Path::new("music.mp3")));
    assert!(!ScanFilter::is_supported_video_format(Path::new("document.pdf")));

    // 核心安全测试：TypeScript 代码文件绝不被识别为视频
    assert!(!ScanFilter::is_supported_video_format(Path::new("index.ts")));
    assert!(!ScanFilter::is_supported_video_format(Path::new("types.d.ts")));
    assert!(!ScanFilter::is_supported_video_format(Path::new("vite.config.ts")));
    assert!(!ScanFilter::is_supported_video_format(Path::new("App.test.ts")));

    // 创建真实的临时 TypeScript 代码文件进行内容级校验
    let temp_dir = tempfile::tempdir().unwrap();
    let ts_file_path = temp_dir.path().join("service.ts");
    std::fs::write(&ts_file_path, b"import React from 'react';\nexport const Api = () => {};").unwrap();
    assert!(!ScanFilter::is_supported_video_format(&ts_file_path));

    // 创建真实的合法 MPEG-TS 视频二进制包（带 0x47 同步字节）进行正向识别测试
    let valid_ts_path = temp_dir.path().join("broadcast.ts");
    let mut ts_bytes = vec![0u8; 564];
    ts_bytes[0] = 0x47;
    ts_bytes[188] = 0x47;
    ts_bytes[376] = 0x47;
    std::fs::write(&valid_ts_path, ts_bytes).unwrap();
    assert!(ScanFilter::is_supported_video_format(&valid_ts_path));
}

#[test]
fn test_cleanup_invalid_typescript_assets() {
    let tmp = tempfile::tempdir().unwrap();
    let db_path = tmp.path().join("test_cleanup.db");
    let mgr = DatabaseManager::new(&db_path).unwrap();

    // 模拟历史扫描误插入了一条 TypeScript 源码文件
    let fake_ts_path = tmp.path().join("index.ts");
    std::fs::write(&fake_ts_path, b"import { useState } from 'react';").unwrap();

    let items = vec![
        VideoAssetRow {
            id: 0,
            file_path: fake_ts_path.to_string_lossy().to_string(),
            file_name: "index.ts".to_string(),
            file_size: 34,
            duration: 0.0,
            width: 0,
            height: 0,
            format: "ts".to_string(),
            created_at: 1700000000,
            modified_at: 1700000000,
            scanned_at: 1700000000,
            status: "discovered".to_string(),
        },
    ];
    mgr.bulk_upsert_assets(&items).unwrap();

    let (total_before, _) = mgr.query_assets(1, 10, None, None, None).unwrap();
    assert_eq!(total_before, 1);

    // 执行清洗函数
    let cleaned = mgr.cleanup_invalid_assets().unwrap();
    assert_eq!(cleaned, 1);

    let (total_after, _) = mgr.query_assets(1, 10, None, None, None).unwrap();
    assert_eq!(total_after, 0);
}
