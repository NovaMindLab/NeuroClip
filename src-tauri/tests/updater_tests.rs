use neuroclip_lib::commands::updater::is_newer_version;

#[test]
fn test_version_comparison_logic() {
    // 补丁版本升级
    assert!(is_newer_version("0.3.1", "0.3.2"));
    assert!(is_newer_version("v0.3.1", "v0.3.2"));
    assert!(is_newer_version("0.3.1", "v0.3.2"));

    // 次版本升级
    assert!(is_newer_version("0.3.1", "0.4.0"));
    assert!(is_newer_version("v0.3.1", "v1.0.0"));

    // 相同版本或更低版本
    assert!(!is_newer_version("0.3.1", "0.3.1"));
    assert!(!is_newer_version("v0.3.1", "0.3.1"));
    assert!(!is_newer_version("0.3.2", "0.3.1"));
    assert!(!is_newer_version("1.0.0", "0.9.9"));
}
