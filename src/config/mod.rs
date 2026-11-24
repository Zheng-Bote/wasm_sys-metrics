// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// General metric configuration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GeneralConfig {
    pub format: String,
    pub prefix: String,
    pub host: String,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            format: "prometheus".to_string(),
            prefix: "sys_".to_string(),
            host: "localhost".to_string(),
        }
    }
}

/// CPU metric collector settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CpuConfig {
    pub enabled: bool,
    pub per_core: bool,
}

impl Default for CpuConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            per_core: true,
        }
    }
}

/// Memory metric collector settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MemoryConfig {
    pub enabled: bool,
    pub swap: bool,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            swap: true,
        }
    }
}

/// Load average collector settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoadAvgConfig {
    pub enabled: bool,
}

impl Default for LoadAvgConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Uptime collector settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UptimeConfig {
    pub enabled: bool,
}

impl Default for UptimeConfig {
    fn default() -> Self {
        Self { enabled: true }
    }
}

/// Disk metric collector settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DiskConfig {
    pub enabled: bool,
    pub mounts: Vec<String>,
}

impl Default for DiskConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            mounts: vec!["/".to_string()],
        }
    }
}

/// Network metric collector settings.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NetworkConfig {
    pub enabled: bool,
    pub interfaces: Vec<String>,
}

impl Default for NetworkConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            interfaces: vec!["eth0".to_string(), "lo".to_string()],
        }
    }
}

/// Complete configuration for system metrics export.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct MetricsConfig {
    pub general: GeneralConfig,
    pub cpu: CpuConfig,
    pub memory: MemoryConfig,
    pub loadavg: LoadAvgConfig,
    pub uptime: UptimeConfig,
    pub disk: DiskConfig,
    pub network: NetworkConfig,
}

impl MetricsConfig {
    /// Loads configuration from a given file path, or falls back to defaults.
    pub fn load_or_default(path_opt: Option<&Path>) -> Self {
        if let Some(path) = path_opt {
            if path.exists() {
                if let Ok(content) = fs::read_to_string(path) {
                    return Self::from_ini_str(&content);
                }
            }
        }

        // Try standard fallback paths
        let standard_paths = [
            Path::new("/config/metrics.ini"),
            Path::new("/config.ini"),
            Path::new("config/metrics.ini"),
            Path::new("metrics.ini"),
        ];

        for standard_path in standard_paths {
            if standard_path.exists() {
                if let Ok(content) = fs::read_to_string(standard_path) {
                    return Self::from_ini_str(&content);
                }
            }
        }

        Self::default()
    }

    /// Parses configuration from an INI formatted string.
    /// Never panics on malformed lines; uses robust defaults.
    pub fn from_ini_str(content: &str) -> Self {
        let mut sections: HashMap<String, HashMap<String, String>> = HashMap::new();
        let mut current_section = "general".to_string();

        for raw_line in content.lines() {
            let line = raw_line.trim();
            if line.is_empty() || line.starts_with(';') || line.starts_with('#') {
                continue;
            }

            if line.starts_with('[') && line.ends_with(']') && line.len() > 2 {
                current_section = line[1..line.len() - 1].trim().to_lowercase();
                sections.entry(current_section.clone()).or_default();
                continue;
            }

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim().to_lowercase();
                let mut val = line[eq_pos + 1..].trim();
                // Strip inline comments if any
                if let Some(comment_pos) = val.find(';') {
                    val = val[..comment_pos].trim();
                }
                if let Some(comment_pos) = val.find('#') {
                    val = val[..comment_pos].trim();
                }
                // Strip optional surrounding quotes
                let val_clean = val.trim_matches('"').trim_matches('\'').trim();

                sections
                    .entry(current_section.clone())
                    .or_default()
                    .insert(key, val_clean.to_string());
            }
        }

        let mut config = Self::default();

        if let Some(sec) = sections.get("general") {
            if let Some(v) = sec.get("format") {
                config.general.format = v.to_lowercase();
            }
            if let Some(v) = sec.get("prefix") {
                config.general.prefix = v.clone();
            }
            if let Some(v) = sec.get("host") {
                config.general.host = v.clone();
            }
        }

        if let Some(sec) = sections.get("cpu") {
            if let Some(v) = sec.get("enabled") {
                config.cpu.enabled = parse_bool(v, config.cpu.enabled);
            }
            if let Some(v) = sec.get("per_core") {
                config.cpu.per_core = parse_bool(v, config.cpu.per_core);
            }
        }

        if let Some(sec) = sections.get("memory") {
            if let Some(v) = sec.get("enabled") {
                config.memory.enabled = parse_bool(v, config.memory.enabled);
            }
            if let Some(v) = sec.get("swap") {
                config.memory.swap = parse_bool(v, config.memory.swap);
            }
        }

        if let Some(sec) = sections.get("loadavg") {
            if let Some(v) = sec.get("enabled") {
                config.loadavg.enabled = parse_bool(v, config.loadavg.enabled);
            }
        }

        if let Some(sec) = sections.get("uptime") {
            if let Some(v) = sec.get("enabled") {
                config.uptime.enabled = parse_bool(v, config.uptime.enabled);
            }
        }

        if let Some(sec) = sections.get("disk") {
            if let Some(v) = sec.get("enabled") {
                config.disk.enabled = parse_bool(v, config.disk.enabled);
            }
            if let Some(v) = sec.get("mounts") {
                config.disk.mounts = parse_list(v);
            }
        }

        if let Some(sec) = sections.get("network") {
            if let Some(v) = sec.get("enabled") {
                config.network.enabled = parse_bool(v, config.network.enabled);
            }
            if let Some(v) = sec.get("interfaces") {
                config.network.interfaces = parse_list(v);
            }
        }

        config
    }
}

fn parse_bool(value: &str, default_val: bool) -> bool {
    match value.to_lowercase().as_str() {
        "true" | "1" | "yes" | "on" => true,
        "false" | "0" | "no" | "off" => false,
        _ => default_val,
    }
}

fn parse_list(value: &str) -> Vec<String> {
    value
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect()
}
