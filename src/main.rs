use std::fs;
use std::path::{Path, PathBuf};

fn main() {
    let home = dirs::home_dir().expect("could not determine home directory");

    println!("Installing Claude Code launcher...\n");

    setup_mcp_config(&home);

    #[cfg(target_os = "macos")]
    install_macos(&home);

    #[cfg(target_os = "linux")]
    install_linux(&home);

    #[cfg(windows)]
    install_windows(&home);

    println!("\nDone.");
}

// ---------------------------------------------------------------------------
// MCP config
// ---------------------------------------------------------------------------

const MCP_CONFIG: &str = r#"{
  "kokoro-tts": {
    "command": "npx",
    "args": ["claude-tts-mcp"]
  },
  "playwright": {
    "command": "npx",
    "args": ["@playwright/mcp@latest"]
  }
}
"#;

const CLAUDE_CMD: &str =
    "claude --allow-dangerously-skip-permissions --mcp-config ~/.config/claude-launcher/mcp.json";

fn setup_mcp_config(home: &Path) {
    let config_dir = home.join(".config").join("claude-launcher");
    fs::create_dir_all(&config_dir).expect("failed to create ~/.config/claude-launcher");
    write_if_changed(&config_dir.join("mcp.json"), MCP_CONFIG);
}

// ---------------------------------------------------------------------------
// macOS — creates ~/Applications/Claude.app
// ---------------------------------------------------------------------------

#[cfg(target_os = "macos")]
fn install_macos(home: &Path) {
    use std::os::unix::fs::PermissionsExt;

    let app = home.join("Applications").join("Claude.app");
    let macos_dir = app.join("Contents").join("MacOS");
    fs::create_dir_all(&macos_dir).expect("failed to create app bundle dirs");

    write_if_changed(&app.join("Contents").join("Info.plist"), MACOS_PLIST);

    let exe = macos_dir.join("claude-launcher");
    let script = format!(
        "#!/bin/bash\nosascript << 'EOF'\ntell application \"Terminal\"\n    activate\n    do script \"cd ~ && {CLAUDE_CMD}\"\nend tell\nEOF\n"
    );
    write_if_changed(&exe, &script);
    fs::set_permissions(&exe, fs::Permissions::from_mode(0o755))
        .expect("failed to chmod launcher");

    println!("  app: {}", app.display());
    println!("\n  Tip: drag ~/Applications/Claude.app to your Dock.");
}

#[cfg(target_os = "macos")]
const MACOS_PLIST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>   <string>claude-launcher</string>
  <key>CFBundleIdentifier</key>  <string>com.claude.launcher</string>
  <key>CFBundleName</key>        <string>Claude</string>
  <key>CFBundlePackageType</key> <string>APPL</string>
  <key>CFBundleVersion</key>     <string>1.0</string>
</dict>
</plist>
"#;

// ---------------------------------------------------------------------------
// Linux — creates ~/.local/share/applications/claude.desktop
// ---------------------------------------------------------------------------

#[cfg(target_os = "linux")]
fn install_linux(home: &Path) {
    use std::os::unix::fs::PermissionsExt;

    // Launcher shell script
    let bin_dir = home.join(".local").join("bin");
    fs::create_dir_all(&bin_dir).expect("failed to create ~/.local/bin");
    let script = bin_dir.join("claude-launcher");
    write_if_changed(
        &script,
        &format!("#!/bin/bash\ncd ~ && {CLAUDE_CMD}\n"),
    );
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
        .expect("failed to chmod launcher");

    // .desktop file
    let desktop_dir = home.join(".local").join("share").join("applications");
    fs::create_dir_all(&desktop_dir).expect("failed to create applications dir");
    let desktop = format!(
        "[Desktop Entry]\nName=Claude\nExec={}\nType=Application\nTerminal=true\nIcon=utilities-terminal\nCategories=Development;\n",
        script.display()
    );
    write_if_changed(&desktop_dir.join("claude.desktop"), &desktop);

    println!("  launcher: {}", script.display());
    println!("  desktop:  {}", desktop_dir.join("claude.desktop").display());
}

// ---------------------------------------------------------------------------
// Windows — creates %LOCALAPPDATA%\claude-launcher\claude.bat
// ---------------------------------------------------------------------------

#[cfg(windows)]
fn install_windows(home: &Path) {
    let dir = home
        .join("AppData")
        .join("Local")
        .join("claude-launcher");
    fs::create_dir_all(&dir).expect("failed to create launcher dir");

    let bat = dir.join("claude.bat");
    let script = format!(
        "@echo off\ncd /d %USERPROFILE%\n{}\n",
        CLAUDE_CMD.replace(
            "~/.config/claude-launcher/mcp.json",
            "%USERPROFILE%\\.config\\claude-launcher\\mcp.json"
        )
    );
    write_if_changed(&bat, &script);

    println!("  launcher: {}", bat.display());
    println!(
        "\n  Tip: create a shortcut to {} and pin it to your taskbar.",
        bat.display()
    );
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Write `content` to `path` only if it differs from the current contents.
fn write_if_changed(path: &PathBuf, content: &str) {
    if path.exists() {
        if fs::read_to_string(path).ok().as_deref() == Some(content) {
            println!("  unchanged  {}", path.display());
            return;
        }
    }
    fs::write(path, content)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
    println!("  written    {}", path.display());
}
