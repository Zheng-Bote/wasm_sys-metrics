# wasm_sys-metrics

Konfigurierbarer System-Metriken-Exporter in Rust als WebAssembly (WASI 0.2 / Component Model) Microservice für Fermyon Spin.

## Inhaltsverzeichnis

- [Features](#features)
- [Installation von Fermyon Spin](#installation-von-fermyon-spin)
- [Konfiguration (config/metrics.ini)](#konfiguration-configmetricsini)
- [Bauen & Kompilieren](#bauen--kompilieren)
- [Starten & Ausführen](#starten--ausführen)
- [Endpunkte](#endpunkte)
- [Entwicklung & Testen](#entwicklung--testen)
- [Lizenz](#lizenz)

---

## Features

- **Per INI konfigurierbar:** Steuerung und Filterung aller Metrik-Kollektoren (`cpu`, `memory`, `loadavg`, `uptime`, `disk`, `network`) über `config/metrics.ini` oder `/config/metrics.ini`.
- **WASM & Spin (Component Model):** Entwickelt gegen das `wasm32-wasip2` Target mit `spin-sdk` v3.
- **Mehrere Ausgabeformate:**
  - Prometheus Text Exposition Format (`text/plain; version=0.0.4`)
  - Strukturiertes JSON (`application/json`)
- **Content Negotiation & Routing:**
  - `GET /metrics` — Prometheus Textformat (Standard) oder JSON via Query `?format=json` bzw. `Accept: application/json`
  - `GET /metrics/json` — Direktes JSON-Format
  - `GET /health` — Status und Uptime (`{"status":"ok", ...}`)
  - `GET /config` — Bereinigte Übersicht aktiver Konfigurationsschalter
- **Graceful Degradation:** Fällt auf sichere Fallbacks zurück, falls Pseudo-Dateisysteme (`/proc`) nicht eingehängt sind (Zero Panic).
- **Spec-Driven Development (SDD):** Entwickelt und verifiziert nach den Qualitätsregeln von `sdd`.

---

## Installation von Fermyon Spin

> [!CAUTION]
> **Wichtig:** Bitte **nicht** `sudo apt install spin` ausführen!
> Das APT-Paket `spin` unter Ubuntu/Debian ist ein formales Modellprüfungs-Werkzeug (Bell Labs Logic Model Checker) und **nicht** die Fermyon Spin WebAssembly CLI.

### Empfohlene Installation

#### Option 1: Offizielles Installationsskript
```bash
curl -fsSL https://developer.fermyon.com/downloads/install.sh | bash
mv spin ~/.local/bin/
```

#### Option 2: Direkt über GitHub Releases
Laden Sie das vorkompilierte Release-Archiv von [GitHub Releases (spinframework/spin)](https://github.com/spinframework/spin/releases) herunter:
```bash
# Beispiel für Linux x86_64:
curl -sL https://github.com/spinframework/spin/releases/download/v4.3.0/spin-v4.3.0-linux-amd64.tar.gz -o /tmp/spin.tar.gz
tar -xzf /tmp/spin.tar.gz -C ~/.local/bin spin
chmod +x ~/.local/bin/spin
rm /tmp/spin.tar.gz
```

#### Installation überprüfen
```bash
spin --version
# Ausgabe z. B.: spin 4.3.0
```

---

## Konfiguration (`config/metrics.ini`)

Das Verhalten des Exporters wird über eine INI-Datei gesteuert:

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

---

## Bauen & Kompilieren

Stellen Sie sicher, dass das Rust-Target `wasm32-wasip2` installiert ist:
```bash
rustup target add wasm32-wasip2
```

Kompilieren mit Spin:
```bash
spin build
```
*(Alternativ via Cargo: `cargo build --target wasm32-wasip2 --release --features spin`)*

Das optimierte WebAssembly-Modul wird unter `target/wasm32-wasip2/release/wasm_sys_metrics.wasm` erzeugt.

---

## Starten & Ausführen

Starten Sie die Spin-Anwendung mit dem Flag `--direct-mounts`:

```bash
spin up --direct-mounts
```

> [!NOTE]
> **Warum `--direct-mounts`?**
> WebAssembly-Komponenten laufen in einer isolierten WASI-Sandbox. Standardmäßig kopiert Spin eingebundene Verzeichnisse in temporäre Verzeichnisse. Da Linux-Kerneldateien in `/proc` virtuelle 0-Byte Streams sind, schlägt ein Kopieren fehl.
> Durch den im Repository hinterlegten Symlink `host_proc -> /proc` und das Flag `--direct-mounts` bindet Spin das Pseudo-Dateisystem direkt in die Sandbox ein, sodass Live-Metriken ausgelesen werden.

---

## Endpunkte

Standardmäßig lauscht Spin auf `http://127.0.0.1:3000`.

### 1. Prometheus-Metriken (`GET /metrics`)
```bash
curl http://127.0.0.1:3000/metrics
```
*Antwort-Header:* `Content-Type: text/plain; version=0.0.4; charset=utf-8`

### 2. JSON-Metriken (`GET /metrics/json` oder `GET /metrics?format=json`)
```bash
curl http://127.0.0.1:3000/metrics/json
# oder
curl -H "Accept: application/json" http://127.0.0.1:3000/metrics
```
*Antwort-Header:* `Content-Type: application/json`

### 3. Health & Uptime Check (`GET /health`)
```bash
curl http://127.0.0.1:3000/health
```
*Beispiel-Antwort:*
```json
{
  "host": "wasm-node",
  "status": "ok",
  "timestamp_ms": 1795531275545,
  "uptime_seconds": 16237.56,
  "version": "0.1.0"
}
```

### 4. Konfigurationsübersicht (`GET /config`)
```bash
curl http://127.0.0.1:3000/config
```
Liefert die aktuell aktiven Module und Einstellungen ohne Offenlegung sensitiver Daten.

---

## Entwicklung & Testen

### Unit- und Integrationstests
```bash
cargo test
```

### Linter & Code-Style
```bash
cargo clippy -- -D warnings
cargo clippy --features spin -- -D warnings
```

### Spec-Driven Development (SDD) Prüfungen
```bash
sdd lint
sdd gate pre-merge
sdd report
```

---

## Lizenz

Apache-2.0
