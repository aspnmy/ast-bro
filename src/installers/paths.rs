use std::path::{Path, PathBuf};

pub fn home() -> Result<PathBuf, String> {
    dirs::home_dir().ok_or_else(|| "cannot resolve user home directory".to_string())
}

pub fn under_home(relative: impl AsRef<Path>) -> Result<PathBuf, String> {
    Ok(home()?.join(relative))
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
