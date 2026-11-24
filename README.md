# wasm_sys-metrics

Konfigurierbarer System-Metriken-Exporter in Rust als WebAssembly (WASI 0.2 / Component Model) Microservice für Fermyon Spin.

## Features

- **Per INI konfigurierbar:** Steuerung von Metrik-Kollektoren (`cpu`, `memory`, `loadavg`, `uptime`, `disk`, `network`) über `config/metrics.ini` oder `/config.ini`.
- **WASM & Spin (Component Model):** Entwickelt gegen das `wasm32-wasip2` Target mit `spin-sdk` v3.
- **Mehrere Ausgabeformate:**
  - Prometheus Text Exposition Format (`text/plain; version=0.0.4`)
  - Strukturiertes JSON (`application/json`)
- **Content Negotiation & Routing:**
  - `GET /metrics` — Prometheus Textformat (Standard) oder JSON via Query `?format=json` bzw. `Accept: application/json`
  - `GET /metrics/json` — Direktes JSON-Format
  - `GET /health` — Status und Uptime (`{"status":"ok", ...}`)
  - `GET /config` — Bereinigte Übersicht aktiver Konfigurationsschalter
- **Graceful Degradation:** Fällt auf sichere Fallbacks zurück, falls Pseudo-Dateisysteme (`/proc`) nicht eingehängt sind.
- **Spec-Driven Development (SDD):** Entwickelt und validiert nach den Regeln von `sdd`.

## Konfiguration (`config/metrics.ini`)

```ini
; Konfiguration für wasm_sys-metrics
[general]
format = prometheus
prefix = sys_
host = wasm-node

[cpu]
enabled = true
per_core = true

[memory]
enabled = true
swap = true

[loadavg]
enabled = true

[uptime]
enabled = true

[disk]
enabled = true
mounts = /

[network]
enabled = true
interfaces = eth0,lo
```

## Entwicklung & Testen

### Tests ausführen
```bash
cargo test
```

### Linter & Code-Style
```bash
cargo clippy -- -D warnings
cargo clippy --features spin -- -D warnings
```

### Build für WebAssembly / Spin
```bash
cargo build --target wasm32-wasip2 --release --features spin
```

Das kompilierte WebAssembly-Modul liegt unter `target/wasm32-wasip2/release/wasm_sys_metrics.wasm`.

### Start mit Fermyon Spin
```bash
spin up --direct-mounts
```
> **Hinweis zur Sandbox:** WebAssembly-Komponenten laufen in einer strikt isolierten WASI-Umgebung. Mit dem Flag `--direct-mounts` und dem im Projekt hinterlegten `host_proc`-Symlink (`host_proc -> /proc`) bindet Spin das Pseudo-Dateisystem des Linux-Hosts live ein, sodass echte Kernel-Statistiken erfasst werden. Ohne Mount greift die Graceful-Degradation mit Nullwerten.

## Lizenz

Apache-2.0
