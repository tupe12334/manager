use std::fs;
use std::path::Path;

const MCP_CONFIG: &str = r#"{
  "mcpServers": {
    "kokoro-tts": {
      "command": "pnpm",
      "args": ["dlx", "claude-tts-mcp"]
    },
    "playwright": {
      "command": "pnpm",
      "args": ["dlx", "@playwright/mcp@latest"]
    }
  }
}
"#;

pub fn setup_mcp_config(home: &Path) {
    let config_dir = home.join(".config").join("claude-launcher");
    fs::create_dir_all(&config_dir).expect("failed to create ~/.config/claude-launcher");
    crate::write_if_changed(&config_dir.join("mcp.json"), MCP_CONFIG);
}
