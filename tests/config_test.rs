// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use wasm_sys_metrics::config::MetricsConfig;

// configurable-system-metrics AC-1: INI parsing, sections, defaults and fallback without panic
#[test]
fn test_ac1_ini_parsing_and_fallbacks() {
    let ini_sample = r#"
    ; Test Configuration
    [general]
    format = json
    prefix = node_
    host = prod-server-1

    [cpu]
    enabled = true
    per_core = false

    [memory]
    enabled = false
    swap = false

    [loadavg]
    enabled = true

    [uptime]
    enabled = true

    [disk]
    enabled = true
    mounts = sda, nvme0n1

    [network]
    enabled = true
    interfaces = eth0, wg0
    "#;

    let cfg = MetricsConfig::from_ini_str(ini_sample);

    assert_eq!(cfg.general.format, "json");
    assert_eq!(cfg.general.prefix, "node_");
    assert_eq!(cfg.general.host, "prod-server-1");
    assert!(cfg.cpu.enabled);
    assert!(!cfg.cpu.per_core);
    assert!(!cfg.memory.enabled);
    assert!(!cfg.memory.swap);
    assert!(cfg.loadavg.enabled);
    assert!(cfg.uptime.enabled);
    assert!(cfg.disk.enabled);
    assert_eq!(cfg.disk.mounts, vec!["sda", "nvme0n1"]);
    assert!(cfg.network.enabled);
    assert_eq!(cfg.network.interfaces, vec!["eth0", "wg0"]);

    // Test malformed content does not panic and uses defaults
    let malformed = "??? not an ini == [[ ]]] invalid";
    let fallback = MetricsConfig::from_ini_str(malformed);
    assert_eq!(fallback.general.format, "prometheus");
    assert_eq!(fallback.general.prefix, "sys_");
    assert!(fallback.cpu.enabled);
}
