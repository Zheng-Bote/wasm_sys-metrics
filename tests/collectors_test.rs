// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use std::fs;
use wasm_sys_metrics::collectors::collect_snapshot;
use wasm_sys_metrics::config::MetricsConfig;

// configurable-system-metrics AC-2: Collectors for CPU, Memory, Loadavg, Uptime, Disk, Net from proc fs
#[test]
fn test_ac2_collectors_from_proc_fs() {
    let temp_dir = std::env::temp_dir().join("wasm_sys_metrics_test_proc");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(temp_dir.join("net")).unwrap();

    // Mock /proc/stat
    fs::write(
        temp_dir.join("stat"),
        "cpu  2255 34 2290 22625563 6290 127 456 0 0 0\n\
         cpu0 1132 17 1145 11312781 3145 63 228 0 0 0\n\
         cpu1 1123 17 1145 11312782 3145 64 228 0 0 0\n",
    )
    .unwrap();

    // Mock /proc/meminfo
    fs::write(
        temp_dir.join("meminfo"),
        "MemTotal:       16384000 kB\n\
         MemFree:         8192000 kB\n\
         MemAvailable:   12000000 kB\n\
         Buffers:          256000 kB\n\
         Cached:          3000000 kB\n\
         SwapTotal:       2097152 kB\n\
         SwapFree:        2097152 kB\n",
    )
    .unwrap();

    // Mock /proc/loadavg
    fs::write(temp_dir.join("loadavg"), "0.45 0.30 0.15 2/350 49123\n").unwrap();

    // Mock /proc/uptime
    fs::write(temp_dir.join("uptime"), "123456.78 987654.32\n").unwrap();

    // Mock /proc/diskstats
    fs::write(
        temp_dir.join("diskstats"),
        "   8       0 sda 1000 50 80000 500 2000 100 160000 1000 0 600 1500\n",
    )
    .unwrap();

    // Mock /proc/net/dev
    fs::write(
        temp_dir.join("net").join("dev"),
        "Inter-|   Receive                                                |  Transmit\n\
         face |bytes    packets errs drop fifo frame compressed multicast|bytes    packets errs drop fifo colls carrier compressed\n\
          eth0: 1048576     500    0    0    0     0          0         0  2097152     800    0    0    0     0       0          0\n",
    )
    .unwrap();

    let mut cfg = MetricsConfig::default();
    cfg.general.host = "test-box".to_string();
    cfg.cpu.per_core = true;
    cfg.disk.mounts = vec!["sda".to_string()];
    cfg.network.interfaces = vec!["eth0".to_string()];

    let snapshot = collect_snapshot(&cfg, &temp_dir);

    // Verify CPU
    let cpu = snapshot.cpu.as_ref().expect("cpu metrics should be present");
    assert_eq!(cpu.total.user, 2255);
    assert_eq!(cpu.total.idle, 22625563);
    assert_eq!(cpu.cores.len(), 2);
    assert_eq!(cpu.cores[0].core_id, 0);
    assert_eq!(cpu.cores[0].times.user, 1132);

    // Verify Memory
    let mem = snapshot.memory.as_ref().expect("memory metrics should be present");
    assert_eq!(mem.total_bytes, 16384000 * 1024);
    assert_eq!(mem.available_bytes, 12000000 * 1024);

    // Verify Loadavg
    let load = snapshot.loadavg.as_ref().expect("loadavg should be present");
    assert_eq!(load.one_min, 0.45);
    assert_eq!(load.runnable_tasks, 2);
    assert_eq!(load.total_tasks, 350);

    // Verify Uptime
    let uptime = snapshot.uptime.as_ref().expect("uptime should be present");
    assert_eq!(uptime.uptime_seconds, 123456.78);

    // Verify Disk
    assert_eq!(snapshot.disk.len(), 1);
    assert_eq!(snapshot.disk[0].device, "sda");
    assert_eq!(snapshot.disk[0].reads_completed, 1000);

    // Verify Network
    assert_eq!(snapshot.network.len(), 1);
    assert_eq!(snapshot.network[0].interface, "eth0");
    assert_eq!(snapshot.network[0].rx_bytes, 1048576);

    let _ = fs::remove_dir_all(&temp_dir);
}

// configurable-system-metrics AC-5: Graceful fallback when proc fs is missing or restricted
#[test]
fn test_ac5_graceful_fallback_without_proc() {
    let non_existent_path = std::path::Path::new("/tmp/non_existent_proc_test_path_12345");
    let cfg = MetricsConfig::default();

    // Must not panic, must return safe fallback structures
    let snapshot = collect_snapshot(&cfg, non_existent_path);

    assert_eq!(snapshot.host, "localhost");
    assert!(snapshot.cpu.is_some());
    assert!(snapshot.memory.is_some());
    assert!(snapshot.loadavg.is_some());
    assert!(snapshot.uptime.is_some());
    assert!(!snapshot.disk.is_empty());
    assert!(!snapshot.network.is_empty());
}

// configurable-system-metrics AC-7: Disk capacity values (SIZE, USED, AVAIL in bytes) from partitions and configs
#[test]
fn test_ac7_disk_capacity_size_used_avail() {
    let temp_dir = std::env::temp_dir().join("wasm_sys_metrics_test_ac7");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    fs::write(
        temp_dir.join("diskstats"),
        "   8       0 sda 100 50 8000 50 200 10 1600 100 0 60 150\n",
    )
    .unwrap();

    fs::write(
        temp_dir.join("partitions"),
        "major minor  #blocks  name\n\
           8     0  976762584 sda\n",
    )
    .unwrap();

    let mut cfg = MetricsConfig::default();
    cfg.disk.mounts = vec!["sda".to_string()];

    let snapshot = collect_snapshot(&cfg, &temp_dir);
    assert_eq!(snapshot.disk.len(), 1);
    assert_eq!(snapshot.disk[0].device, "sda");
    assert_eq!(snapshot.disk[0].size_bytes, Some(976762584 * 1024));

    let _ = fs::remove_dir_all(&temp_dir);
}
