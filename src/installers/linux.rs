use std::fs;
use std::path::Path;

pub fn install_linux(home: &Path, claude_cmd: &str) {
    use std::os::unix::fs::PermissionsExt;

    // Launcher shell script
    let bin_dir = home.join(".local").join("bin");
    fs::create_dir_all(&bin_dir).expect("failed to create ~/.local/bin");
    let script = bin_dir.join("claude-launcher");
    crate::write_if_changed(
        &script,
        &format!("#!/bin/bash\ncd ~ && {claude_cmd}\n"),
    );
    fs::set_permissions(&script, fs::Permissions::from_mode(0o755))
        .expect("failed to chmod launcher");

    // .desktop file
    let desktop_dir = home.join(".local").join("share").join("applications");
    fs::create_dir_all(&desktop_dir).expect("failed to create applications dir");
    let desktop = format!(
        "[Desktop Entry]\nName=Manager\nExec={}\nType=Application\nTerminal=true\nIcon=utilities-terminal\nCategories=Development;\n",
        script.display()
    );
    crate::write_if_changed(&desktop_dir.join("claude.desktop"), &desktop);

    println!("  launcher: {}", script.display());
    println!("  desktop:  {}", desktop_dir.join("claude.desktop").display());
}
