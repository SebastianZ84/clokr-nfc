# Clokr NFC Client

Native desktop app for NFC-based time tracking with [Clokr](https://github.com/SebastianZ84/clokr).

## Features

- **System tray** — runs in the background, no window needed
- **PC/SC smart card reader** support (e.g., SCM uTrust 3700 F)
- **Toast notifications** — instant feedback on clock in/out
- **Offline queue** — punches stored locally when API is unreachable, auto-retry every 30s
- **Auto-start** — optionally start with OS
- **Sound feedback** — configurable audio on successful punch

## Supported Hardware

Any USB NFC reader that supports PC/SC protocol with 13.56 MHz MIFARE Classic cards:

- SCM uTrust 3700 F (recommended)
- ACR122U
- HID Omnikey 5022/5427

## Setup

1. Download the latest release for your OS from [Releases](https://github.com/SebastianZ84/clokr-nfc/releases)
2. Install and launch
3. Right-click the tray icon → Settings
4. Enter your Clokr API URL (e.g., `https://clokr.yourcompany.de/api/v1`)
5. Enter the API Key (create one in Clokr Admin → System → NFC-Terminals)
6. Scan an NFC card to test

## Development

### Prerequisites

- [Rust](https://rustup.rs/) (latest stable)
- [Node.js](https://nodejs.org/) 24+
- [pnpm](https://pnpm.io/) 10+
- PC/SC library (`pcsclite` on Linux, built-in on macOS/Windows)

### Run

```bash
pnpm install
pnpm tauri dev
```

### Build

```bash
pnpm tauri build
```

Output: `src-tauri/target/release/bundle/` (DMG on macOS, MSI on Windows)

## How It Works

1. App listens for NFC card scans via PC/SC API
2. On card detected → reads UID (e.g., `04:A2:B3:C4:D5:E6:F7`)
3. Sends `POST /api/v1/time-entries/nfc-punch` with `{ nfcCardId, terminalSecret }`
4. API toggles clock in/out based on current state
5. Shows native OS notification with employee name + action

## Related

- [Clokr](https://github.com/SebastianZ84/clokr) — Time tracking & team management (main project)
- Browser-based terminal available at `/terminal` in the Clokr web app (no installation needed)

## License

MIT
