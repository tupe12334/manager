# manager

Idempotent cross-platform installer for a Claude Code launcher app (macOS, Linux, Windows).

Running it sets up:

- `~/.config/claude-launcher/mcp.json` — MCP server config (`kokoro-tts`, `playwright`).
- `~/.claude/hooks/speak-response.sh` and a `Stop` hook entry in `~/.claude/settings.json` — reads
  the assistant's last message and asks Claude to speak it via `kokoro-tts`, but only when running
  inside a session launched by this app.
- A per-OS launcher for the `claude --allow-dangerously-skip-permissions --mcp-config ~/.config/claude-launcher/mcp.json` command:
  - **macOS** — an app bundle at `~/Applications/Manager.app` that opens Terminal and runs the command.
  - **Linux** — a script at `~/.local/bin/claude-launcher` plus a `~/.local/share/applications/claude.desktop` entry.
  - **Windows** — `%USERPROFILE%\AppData\Local\claude-launcher\claude.bat`.

Every write is idempotent: files are only rewritten when their content would actually change.

## Build & run

Requires a Rust toolchain (edition 2024).

```sh
cargo run --bin install
```

or build a release binary and run it directly:

```sh
cargo build --release
./target/release/install
```

The installer takes no arguments or environment variables — it always installs for the current OS
into the paths listed above.
