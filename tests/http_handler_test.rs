// Copyright (c) 2026 Zheng-Bote
// SPDX-License-Identifier: Apache-2.0

use std::path::Path;
use wasm_sys_metrics::config::MetricsConfig;
use wasm_sys_metrics::handle_request_route;

// configurable-system-metrics AC-4: Spin HTTP handler routes and negotiation for /metrics, /health, /config
#[test]
fn test_ac4_http_routes_and_negotiation() {
    let config = MetricsConfig::default();
    let proc_root = Path::new("/proc");

    // 1. GET /metrics (default prometheus)
    let res = handle_request_route("GET", "/metrics", None, None, &config, proc_root);
    assert_eq!(res.status, 200);
    assert_eq!(
        res.headers[0].1,
        "text/plain; version=0.0.4; charset=utf-8"
    );
    let body = String::from_utf8(res.body).unwrap();
    assert!(body.contains("sys_"));

    // 2. GET /metrics?format=json
    let res_json = handle_request_route(
        "GET",
        "/metrics",
        Some("format=json"),
        None,
        &config,
        proc_root,
    );
    assert_eq!(res_json.status, 200);
    assert_eq!(res_json.headers[0].1, "application/json");

    // 3. GET /metrics with Accept: application/json
    let res_accept = handle_request_route(
        "GET",
        "/metrics",
        None,
        Some("application/json"),
        &config,
        proc_root,
    );
    assert_eq!(res_accept.status, 200);
    assert_eq!(res_accept.headers[0].1, "application/json");

    // 4. GET /metrics/json
    let res_direct_json =
        handle_request_route("GET", "/metrics/json", None, None, &config, proc_root);
    assert_eq!(res_direct_json.status, 200);
    assert_eq!(res_direct_json.headers[0].1, "application/json");

    // 5. GET /health
    let res_health = handle_request_route("GET", "/health", None, None, &config, proc_root);
    assert_eq!(res_health.status, 200);
    let health_body = String::from_utf8(res_health.body).unwrap();
    assert!(health_body.contains("\"status\":\"ok\"") || health_body.contains("\"status\": \"ok\""));

    // 6. GET /config
    let res_cfg = handle_request_route("GET", "/config", None, None, &config, proc_root);
    assert_eq!(res_cfg.status, 200);
    let cfg_body = String::from_utf8(res_cfg.body).unwrap();
    assert!(cfg_body.contains("prometheus"));

    // 7. Unknown route -> 404
    let res_404 = handle_request_route("GET", "/invalid", None, None, &config, proc_root);
    assert_eq!(res_404.status, 404);

    // 8. POST method -> 405
    let res_405 = handle_request_route("POST", "/metrics", None, None, &config, proc_root);
    assert_eq!(res_405.status, 405);
}
