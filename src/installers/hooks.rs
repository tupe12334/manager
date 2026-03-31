use std::fs;
use std::path::Path;

const SPEAK_SCRIPT: &str = r#"#!/bin/bash
# Stop hook: wakes the current Claude session (asyncRewake) and asks it to speak
# the last response using the kokoro-tts MCP tool.
# Only runs when launched via the Manager app (MANAGER_SESSION=1).
# The lock file prevents re-triggering after Claude has spoken.
[ -z "$MANAGER_SESSION" ] && exit 0
SESSION=$(cat | jq -r '.session_id // "unknown"' 2>/dev/null)
LOCK="/tmp/claude-tts-spoke-${SESSION}"
if [ -f "$LOCK" ]; then
  rm "$LOCK"
  exit 0
fi
touch "$LOCK"
printf "Call the mcp__kokoro-tts__generate_speech tool to speak your last response aloud."
exit 2
"#;

pub fn setup_hooks(home: &Path) {
    let hooks_dir = home.join(".claude").join("hooks");
    fs::create_dir_all(&hooks_dir).expect("failed to create ~/.claude/hooks");

    let script_path = hooks_dir.join("speak-response.sh");
    crate::write_if_changed(&script_path, SPEAK_SCRIPT);

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&script_path, fs::Permissions::from_mode(0o755))
            .expect("failed to chmod speak-response.sh");
    }

    setup_stop_hook(home);
}

fn setup_stop_hook(home: &Path) {
    let settings_path = home.join(".claude").join("settings.json");

    let mut settings: serde_json::Value = if settings_path.exists() {
        let content =
            fs::read_to_string(&settings_path).expect("failed to read ~/.claude/settings.json");
        serde_json::from_str(&content).unwrap_or(serde_json::json!({}))
    } else {
        serde_json::json!({})
    };

    settings["hooks"]["Stop"] = serde_json::json!([{
        "hooks": [{
            "type": "command",
            "command": "bash ~/.claude/hooks/speak-response.sh",
            "asyncRewake": true,
            "timeout": 10
        }]
    }]);

    let new_content =
        serde_json::to_string_pretty(&settings).expect("failed to serialize settings.json") + "\n";

    crate::write_if_changed(&settings_path, &new_content);
}
