---
feature: configurable-system-metrics
status: in_progress
archive-after: 2027-01-01
---

# Plan — Konfigurierbare System-Metriken per Abruf (`configurable-system-metrics`)

Reihenfolge der Umsetzung. Jede Task nennt das Akzeptanzkriterium, auf das sie einzahlt.

## Tasks

- [x] AC-1: Cargo Projekt initialisieren und INI Konfigurationsparser mit robusten Standardwerten implementieren
- [x] AC-2: Modulare Metrik-Kollektoren für CPU, Memory, LoadAvg, Uptime, Disk und Netzwerk basierend auf `/proc` Filesystem implementieren
- [x] AC-3: Formatierungsmodule für Prometheus Text Exposition Format und JSON Serialisierung implementieren
- [x] AC-4: Spin HTTP Handler mit Routen-Dispatching für `/metrics`, `/metrics/json`, `/health` und `/config` implementieren
- [x] AC-5: Fehlertolerante Fallback-Strategie und saubere Fehlerbehandlung für Umgebungen ohne `/proc` Filesystem absichern
- [x] AC-6: `spin.toml` konfigurieren, Integrationstests erstellen und WASM-Kompilierung für `wasm32-wasip2` validieren
