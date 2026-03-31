use std::fs;
use std::path::Path;

const MACOS_PLIST: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>   <string>claude-launcher</string>
  <key>CFBundleIdentifier</key>  <string>com.manager.launcher</string>
  <key>CFBundleName</key>        <string>Manager</string>
  <key>CFBundlePackageType</key> <string>APPL</string>
  <key>CFBundleVersion</key>     <string>1.0</string>
</dict>
</plist>
"#;

pub fn install_macos(home: &Path, claude_cmd: &str) {
    use std::os::unix::fs::PermissionsExt;

    let app = home.join("Applications").join("Manager.app");
    let macos_dir = app.join("Contents").join("MacOS");
    fs::create_dir_all(&macos_dir).expect("failed to create app bundle dirs");

    crate::write_if_changed(&app.join("Contents").join("Info.plist"), MACOS_PLIST);

    let exe = macos_dir.join("claude-launcher");
    let script = format!(
        "#!/bin/bash\nosascript << 'EOF'\ntell application \"Terminal\"\n    activate\n    do script \"cd ~ && {claude_cmd}\"\nend tell\nEOF\n"
    );
    crate::write_if_changed(&exe, &script);
    fs::set_permissions(&exe, fs::Permissions::from_mode(0o755))
        .expect("failed to chmod launcher");

    println!("  app: {}", app.display());
    println!("\n  Tip: drag ~/Applications/Claude.app to your Dock.");
}
