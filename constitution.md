---
status: active
---

# Verfassung — wasm_sys-metrics

Diese Verfassung enthält die systemweiten, dauerhaften Regeln des Projekts. Feature-Inhalte (User Stories, Tasklisten, Planung) gehören in PRD, Tech Spec und Plan. Jede Änderung einer Regel verweist auf ein ADR unter `docs/adr`.

## Architektur/Layering

1. **Konfigurationsschicht (`src/config/`):**
   - Zuständig für das Einlesen, Validieren und typisierte Bereitstellen von Systemeinstellungen (z. B. aus INI-Dateien).
   - Unabhängig von Spin-HTTP-Typen; rein deklarativ und zustandslos.

2. **Kollektorschicht (`src/collectors/`):**
   - Liest Systemmetriken (CPU, Speicher, Disk, LoadAvg, Uptime, Netzwerk) aus pseudo-Dateisystemen (z. B. `/proc`, `/sys`) oder Mock-Quellen aus.
   - Darf keine HTTP-Logik enthalten. Liefert typisierte Metrik-Snapshots.

3. **Formatierungsschicht (`src/formatters/`):**
   - Wandelt Metrik-Snapshots in Zielformate um (Prometheus Text Exposition Format, JSON).
   - Reine Transformation ohne Seiteneffekte und I/O.

4. **Spin HTTP Adapter / Entrypoint (`src/lib.rs`):**
   - Verknüpft Spin HTTP Triggers mit Konfiguration, Kollektoren und Formatierern.
   - Behandelt Routen (`/metrics`, `/health`, `/config`).
   - Schreibt keine Geschäftslogik direkt in den Handler.

## Security

1. **Read-Only Zugriff:** Metrik-Erfassung operiert strikt lesend auf freigegebenen Pseudodateien (`/proc`, `/sys`). Keine destruktiven Dateioperationen oder Subprozessausführungen.
2. **Keine Secret-Exposition:** Konfigurations- oder Diagnoseendpunkte dürfen keine Passwörter, Token oder vertrauliche Host-Informationen leaken.
3. **Resilienz & Denial-of-Service-Schutz:** Begrenzung von Puffergrößen bei Dateilesevorgängen; Schutz vor unendlichen Schleifen oder unkontrolliertem Speicherverbrauch.

## Strikte Regeln

1. **Zero Panic Policy:** Keine `unwrap()` oder `expect()` auf externen oder unsicheren I/O-Eingaben (Dateisystem, Netzwerk, HTTP-Body). Jeder I/O- oder Parse-Fehler wird via `Result` abgefangen.
2. **WASM / Spin Kompatibilität:** Der Kern und HTTP-Adapter müssen stets für das Target `wasm32-wasip2` fehlerfrei kompilieren.
3. **Qualitätstore:** Vor jedem Merge müssen `cargo test`, `cargo clippy -- -D warnings`, `sdd lint` und das jeweilige `sdd gate` ohne Befunde durchlaufen.
4. **Lizenz & Dokumentation:** Alle Quellcode-Dateien halten den SPDX-Header `Apache-2.0` ein.
