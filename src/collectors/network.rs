// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use crate::models::NetworkMetrics;

pub fn parse_net_dev(content: &str, target_interfaces: &[String]) -> Vec<NetworkMetrics> {
    let mut results = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if !line.contains(':') {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 2 {
            continue;
        }

        let iface = parts[0].trim().to_string();
        if !target_interfaces.is_empty()
            && !target_interfaces.iter().any(|i| i == "*" || i == &iface)
        {
            continue;
        }

        let fields: Vec<&str> = parts[1].split_whitespace().collect();
        if fields.len() >= 16 {
            let rx_bytes = fields[0].parse::<u64>().unwrap_or(0);
            let rx_packets = fields[1].parse::<u64>().unwrap_or(0);
            let rx_errors = fields[2].parse::<u64>().unwrap_or(0);
            let rx_dropped = fields[3].parse::<u64>().unwrap_or(0);

            let tx_bytes = fields[8].parse::<u64>().unwrap_or(0);
            let tx_packets = fields[9].parse::<u64>().unwrap_or(0);
            let tx_errors = fields[10].parse::<u64>().unwrap_or(0);
            let tx_dropped = fields[11].parse::<u64>().unwrap_or(0);

            results.push(NetworkMetrics {
                interface: iface,
                rx_bytes,
                rx_packets,
                rx_errors,
                rx_dropped,
                tx_bytes,
                tx_packets,
                tx_errors,
                tx_dropped,
            });
        }
    }

    results
}
