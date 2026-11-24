// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use std::path::Path;

// configurable-system-metrics AC-6: Spin manifest spin.toml and target wasm32-wasip2 readiness
#[test]
fn test_ac6_wasm_target_and_spin_manifest() {
    let manifest_path = Path::new("spin.toml");
    assert!(manifest_path.exists(), "spin.toml must exist");

    let content = fs::read_to_string(manifest_path).expect("spin.toml must be readable");
    assert!(content.contains("spin_manifest_version = 2"));
    assert!(content.contains("[[trigger.http]]"));
    assert!(content.contains("wasm32-wasip2"));
    assert!(content.contains("/proc"));
}
