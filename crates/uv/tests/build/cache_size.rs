use assert_cmd::assert::OutputAssertExt;

use uv_test::uv_snapshot;

/// Test that `cache size` returns a human-readable size for an empty cache directory.
#[test]
fn cache_size_empty() {
    let context = uv_test::test_context!("3.12");

    // Clean cache first to ensure truly empty state
    context.clean().assert().success();

    uv_snapshot!(context.cache_size().arg("--preview"), @"
    exit_code: 0 (success)
    ----- stdout -----
    [SIZE]
    ");
}

/// Test that `cache size --bytes` returns 0 for an empty cache directory.
#[test]
fn cache_size_empty_bytes() {
    let context = uv_test::test_context!("3.12");

    // Clean cache first to ensure truly empty state
    context.clean().assert().success();

    uv_snapshot!(context.cache_size().arg("--preview").arg("--bytes"), @"
    exit_code: 0 (success)
    ----- stdout -----
    0
    ");
}

/// Test that `cache size` returns a human-readable size after installing packages.
#[test]
fn cache_size_with_packages() {
    let context = uv_test::test_context!("3.12");

    // Install a requirement to populate the cache.
    context.pip_install().arg("iniconfig").assert().success();

    // Check cache size is now positive (human-readable, the default).
    uv_snapshot!(context.with_filtered_cache_size().filters(), context.cache_size().arg("--preview"), @"
    exit_code: 0 (success)
    ----- stdout -----
    [SIZE]
    ");
}

/// Test that `cache size --bytes` returns raw bytes after installing packages.
#[test]
fn cache_size_with_packages_bytes() {
    let context = uv_test::test_context!("3.12");

    // Install a requirement to populate the cache.
    context.pip_install().arg("iniconfig").assert().success();

    // Check cache size with the `--bytes` flag.
    uv_snapshot!(context.with_filtered_cache_size().filters(), context.cache_size().arg("--preview").arg("--bytes"), @"
    exit_code: 0 (success)
    ----- stdout -----
    [SIZE]
    ");
}

/// Test that `cache size --human` is accepted and matches the default output.
#[test]
fn cache_size_with_packages_human() {
    let context = uv_test::test_context!("3.12");

    // Install a requirement to populate the cache.
    context.pip_install().arg("iniconfig").assert().success();

    // Check cache size with the `--human` flag.
    uv_snapshot!(context.with_filtered_cache_size().filters(), context.cache_size().arg("--preview").arg("--human"), @"
    exit_code: 0 (success)
    ----- stdout -----
    [SIZE]
    ");
}

/// Test that `--human` and `--bytes` conflict.
#[test]
fn cache_size_human_bytes_conflict() {
    let context = uv_test::test_context!("3.12");

    uv_snapshot!(context.cache_size().arg("--preview").arg("--human").arg("--bytes"), @"
    exit_code: 2 (failure)
    ----- stderr -----
    error: the argument '--human' cannot be used with '--bytes'

    Usage: uv cache size --cache-dir [CACHE_DIR] --human

    For more information, try '--help'.
    ");
}
