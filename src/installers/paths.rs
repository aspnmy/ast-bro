use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

pub fn home() -> Result<PathBuf, String> {
    dirs::home_dir().ok_or_else(|| "cannot resolve user home directory".to_string())
}

pub fn under_home(relative: impl AsRef<Path>) -> Result<PathBuf, String> {
    Ok(home()?.join(relative))
}

/// True if `name` resolves to an executable on the user's $PATH. Used
/// by adapters' `detect()` so `--all` can skip CLIs the user doesn't
/// actually have installed.
pub fn binary_on_path(name: &str) -> bool {
    Command::new("which")
        .arg(name)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn under_home_joins_relative() {
        let p = under_home(".claude/CLAUDE.md").unwrap();
        assert!(p.ends_with(".claude/CLAUDE.md"));
        assert!(p.is_absolute());
    }
}
