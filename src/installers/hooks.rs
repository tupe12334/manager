use std::fs;
use std::path::Path;

const SPEAK_SCRIPT: &str = r#"#!/bin/bash
# Stop hook: wakes the current Claude session (asyncRewake) with the exact text
# to speak, so Claude can call mcp__kokoro-tts__generate_speech directly.
# Only runs when launched via the Manager app (detected via ancestor process args).

# --- Manager session detection ---
IS_MANAGER=0
pid=$PPID
for i in 1 2 3 4 5; do
  cmd=$(ps -o command= -p "$pid" 2>/dev/null) || break
  if echo "$cmd" | grep -q "claude-launcher/mcp.json"; then
    IS_MANAGER=1; break
  fi
  pid=$(ps -o ppid= -p "$pid" 2>/dev/null | tr -d ' ') || break
  [ "${pid:-0}" -le 1 ] 2>/dev/null && break
done
[ "$IS_MANAGER" -eq 0 ] && exit 0

# --- Read session data ---
DATA=$(cat)
SESSION=$(echo "$DATA" | jq -r '.session_id // ""' 2>/dev/null)

# --- Lock file prevents re-triggering after Claude has spoken ---
LOCK="/tmp/claude-tts-spoke-${SESSION:-unknown}"
if [ -f "$LOCK" ]; then
  rm "$LOCK"
  exit 0
fi
touch "$LOCK"

# --- Extract last assistant message from session JSONL ---
TEXT=""
if [ -n "$SESSION" ]; then
  JSONL=$(find ~/.claude/projects -name "${SESSION}.jsonl" 2>/dev/null | head -1)
  if [ -n "$JSONL" ]; then
    TEXT=$(jq -r 'select(.message.role == "assistant") | .message.content[]? | select(.type == "text") | .text' "$JSONL" 2>/dev/null | tail -c 800)
  fi
fi

if [ -z "$TEXT" ]; then
  exit 0
fi

printf 'Call mcp__kokoro-tts__generate_speech with play_audio=true and this exact text: %s' "$TEXT"
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
