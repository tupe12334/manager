use std::fs;
use std::path::Path;

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

pub fn setup_mcp_config(home: &Path) {
    let config_dir = home.join(".config").join("claude-launcher");
    fs::create_dir_all(&config_dir).expect("failed to create ~/.config/claude-launcher");
    crate::write_if_changed(&config_dir.join("mcp.json"), MCP_CONFIG);
}
