use std::fs;
use std::path::Path;

pub fn install_windows(home: &Path, claude_cmd: &str) {
    let dir = home
        .join("AppData")
        .join("Local")
        .join("claude-launcher");
    fs::create_dir_all(&dir).expect("failed to create launcher dir");

    let bat = dir.join("claude.bat");
    let script = format!(
        "@echo off\ncd /d %USERPROFILE%\n{}\n",
        claude_cmd.replace(
            "~/.config/claude-launcher/mcp.json",
            "%USERPROFILE%\\.config\\claude-launcher\\mcp.json"
        )
    );
    crate::write_if_changed(&bat, &script);

    println!("  launcher: {}", bat.display());
    println!(
        "\n  Tip: create a shortcut to {} and pin it to your taskbar.",
        bat.display()
    );
}
