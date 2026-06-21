use std::fs;
use std::path::Path;

mod installers;

const CLAUDE_CMD: &str =
    "claude --allow-dangerously-skip-permissions --mcp-config ~/.config/claude-launcher/mcp.json";

fn main() {
    let home = dirs::home_dir().expect("could not determine home directory");

    println!("Installing Claude Code launcher...\n");

    installers::mcp::setup_mcp_config(&home);
    installers::hooks::setup_hooks(&home);

    #[cfg(target_os = "macos")]
    installers::macos::install_macos(&home, CLAUDE_CMD);

    #[cfg(target_os = "linux")]
    installers::linux::install_linux(&home, CLAUDE_CMD);

    #[cfg(windows)]
    installers::windows::install_windows(&home, CLAUDE_CMD);

    println!("\nDone.");
}

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

/// Write `content` to `path` only if it differs from the current contents.
///
/// # Panics
///
/// Panics if writing to `path` fails (for example, when the parent directory
/// does not exist or the process lacks permission to write there).
pub fn write_if_changed(path: &Path, content: &str) {
    if path.exists() && fs::read_to_string(path).ok().as_deref() == Some(content) {
        println!("  unchanged  {}", path.display());
        return;
    }
    fs::write(path, content)
        .unwrap_or_else(|e| panic!("failed to write {}: {e}", path.display()));
    println!("  written    {}", path.display());
}
