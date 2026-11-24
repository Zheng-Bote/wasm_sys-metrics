---
feature: configurable-system-metrics
status: in_progress
archive-after: 2027-01-01
touches:
  - "Cargo.toml"
  - "spin.toml"
  - "src/**"
  - "tests/**"
  - "config/**"
---

# Tech Spec — Konfigurierbare System-Metriken per Abruf (`configurable-system-metrics`)

Setzt das PRD `docs/prd/configurable-system-metrics.md` um. Die Verfassung hat Vorrang; diese Tech Spec darf sie präzisieren, nie lockern.

## Architektur

Das System folgt der in `constitution.md` festgelegten Schichtenarchitektur:

```text
                  +--------------------------------+
                  |   Spin HTTP Trigger (HTTP)     |
                  |          (src/lib.rs)          |
                  +---------------+----------------+
                                  |
                                  v
     +----------------------------+----------------------------+
     |                            |                            |
     v                            v                            v
+------------------+    +-------------------+    +----------------------+
| src/config/      |    | src/collectors/   |    | src/formatters/      |
|  - INI Parser    |    |  - CpuCollector   |    |  - Prometheus Formatter
|  - Config Struct |    |  - MemCollector   |    |  - JSON Formatter    |
|  - Defaults      |    |  - LoadAvgColl.   |    +----------------------+
+------------------+    |  - UptimeColl.    |
                        |  - DiskCollector  |
                        |  - NetCollector   |
                        +---------+---------+
                                  |
                                  v
                       /proc, /sys or Fallback
```

1. **Config Layer (`src/config/mod.rs`):**
   - Parst INI-Dateiformat via Pure-Rust Parser (`ini` oder interner Tokenizer).
   - Erzeugt `MetricsConfig`: Globale Einstellungen (Präfix, Standardformat, Hostname) und modulspezifische Flags (`cpu.enabled`, `cpu.per_cpu`, `memory.enabled`, `memory.swap`, `disk.enabled`, `disk.mounts`, `network.enabled`, `network.interfaces`, `loadavg.enabled`).
   - Standardwerte greifen sofort, falls die INI fehlt oder fehlerhaft ist.

2. **Collector Layer (`src/collectors/mod.rs`):**
   - `Collector` Trait oder funktionale Kollektoren: `collect(&self, config: &MetricsConfig, proc_root: &Path) -> Result<SystemMetricsSnapshot, CollectorError>`.
   - Modulare Unterkollektoren für:
     - CPU: `/proc/stat` (Gesamtauslastung in User, System, Idle, IOWait, etc.; optional per Core).
     - Memory: `/proc/meminfo` (Total, Free, Available, Buffers, Cached, SwapTotal, SwapFree).
     - LoadAvg: `/proc/loadavg` (1m, 5m, 15m, runnable/total entities).
     - Uptime: `/proc/uptime` (Uptime in Sekunden, Idle-Zeit).
     - Disk: `/proc/diskstats` oder gemappte Dateisystemstatistiken.
     - Network: `/proc/net/dev` (Empfangene/gesendete Bytes und Pakete je Interface).

3. **Formatter Layer (`src/formatters/mod.rs`):**
   - `PrometheusFormatter`: Generiert Zeilen nach OpenMetrics / Prometheus Text Standard (`# HELP`, `# TYPE`, Metrikname mit Labels und Timestamps).
   - `JsonFormatter`: Generiert ein sauberes JSON-Objekt mit Metriken nach Kategorien.

4. **Spin HTTP Adapter (`src/lib.rs`):**
   - Nutzt `spin_sdk::http::{Request, Response, Method}` bzw. Spin v3 Component Model.
   - Routen-Matching:
     - `GET /metrics` -> prüft `Accept` Header (`application/json` vs `text/plain`) oder Query-Param `format=json|prometheus`.
     - `GET /metrics/json` -> JSON Format.
     - `GET /health` -> `{"status": "ok", "uptime_seconds": ..., "version": ...}`.
     - `GET /config` -> JSON Repräsentation der aktiven Metrik-Schalter.
     - Alles andere -> 404 Not Found.

## Schnittstellen

- **HTTP GET `/metrics`**:
  - Response 200 `Content-Type: text/plain; version=0.0.4; charset=utf-8` (Standard)
  - Oder `Content-Type: application/json` bei `?format=json`
- **HTTP GET `/metrics/json`**:
  - Response 200 `Content-Type: application/json`
- **HTTP GET `/health`**:
  - Response 200 `Content-Type: application/json`
- **HTTP GET `/config`**:
  - Response 200 `Content-Type: application/json`
- **Konfigurationsdatei `config/metrics.ini`**:
  - Standardpfad konfigurierbar via Umgebungsvariable `METRICS_CONFIG_PATH` oder Standard `/config.ini` bzw. `config/metrics.ini`.

## Datenmodell

```rust
pub struct MetricsConfig {
    pub general: GeneralConfig,
    pub cpu: CpuConfig,
    pub memory: MemoryConfig,
    pub disk: DiskConfig,
    pub loadavg: LoadAvgConfig,
    pub network: NetworkConfig,
}

pub struct SystemMetricsSnapshot {
    pub timestamp_ms: u64,
    pub host: String,
    pub cpu: Option<CpuMetrics>,
    pub memory: Option<MemoryMetrics>,
    pub loadavg: Option<LoadAvgMetrics>,
    pub uptime: Option<UptimeMetrics>,
    pub disk: Vec<DiskMetrics>,
    pub network: Vec<NetworkMetrics>,
}
```

## Umsetzung der Akzeptanzkriterien

| AC | Umsetzung | Test |
|---|---|---|
| AC-1 | `src/config/mod.rs`: INI Parsing, Default-Werte, Validierung | `tests/config_test.rs::test_ac1_ini_parsing_and_fallbacks` |
| AC-2 | `src/collectors/`: Linux /proc Dateiparser (CPU, Mem, Load, Disk, Net) | `tests/collectors_test.rs::test_ac2_collectors_from_proc_fs` |
| AC-3 | `src/formatters/`: Prometheus Exporter & JSON Serializer | `tests/formatters_test.rs::test_ac3_formatters_output` |
| AC-4 | `src/lib.rs`: Spin HTTP Request Routing (`/metrics`, `/health`, etc.) | `tests/http_handler_test.rs::test_ac4_http_routes_and_negotiation` |
| AC-5 | `src/collectors/mod.rs` & `src/lib.rs`: Graceful Degradation & Safe Fallback | `tests/collectors_test.rs::test_ac5_graceful_fallback_without_proc` |
| AC-6 | `spin.toml`, `Cargo.toml`, Target `wasm32-wasip2` | `tests/build_test.rs::test_ac6_wasm_target_and_spin_manifest` |

## Sicherheit

- **Read-Only:** Alle Dateioperationen in `src/collectors/` und `src/config/` nutzen strikt `File::open` und `read_to_string` mit Pufferbegrenzung (max 64 KB je Datei).
- **Zero Panic:** Niemals `unwrap()` oder `expect()` auf Datei- oder Parse-Inhalten.
- **Keine sensible Offenlegung:** `/config` und `/health` listen nur boolesche Schalter und Zähler, keine Dateipfade oder Umgebungsvariablen.

## Offene Punkte

- Keine offenen Punkte; Spin 3 / WASI 0.2 `spin.toml` Manifeste erlauben das Einhängen lokaler Verzeichnisse wie `/proc` via `files = [...]`.
