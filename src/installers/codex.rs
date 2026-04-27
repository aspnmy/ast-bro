use std::path::PathBuf;

use super::paths;
use super::{common, Change, Detection, InstallOpts, Installer, Scope, Status};
use crate::prompt::AGENT_PROMPT;

pub struct Codex;

impl Codex {
    fn prompt_path(&self, scope: &Scope) -> Result<PathBuf, String> {
        match scope {
            Scope::Local(root) => Ok(root.join("AGENTS.md")),
            Scope::Global => paths::under_home(".codex/AGENTS.md"),
        }
    }
}

impl Installer for Codex {
    fn name(&self) -> &'static str {
        "codex"
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let dir_exists = self
            .prompt_path(scope)
            .ok()
            .and_then(|p| p.parent().map(|r| r.to_path_buf()))
            .map(|r| r.exists())
            .unwrap_or(false);
        Detection {
            present: dir_exists || paths::binary_on_path("codex"),
        }
    }

    fn install_prompt(&self, scope: &Scope, opts: &InstallOpts) -> Result<Change, String> {
        common::install_prompt_in(&self.prompt_path(scope)?, AGENT_PROMPT, opts)
    }

    fn install_hook(&self, _scope: &Scope, _opts: &InstallOpts) -> Result<Change, String> {
        Ok(Change::NotApplicable)
    }

    fn uninstall(&self, scope: &Scope, opts: &InstallOpts) -> Result<Vec<Change>, String> {
        let mut changes = Vec::new();
        if let Some(c) = common::uninstall_prompt_in(&self.prompt_path(scope)?, opts)? {
            changes.push(c);
        }
        Ok(changes)
    }

    fn status(&self, scope: &Scope) -> Status {
        common::status_for_prompt_only(self.prompt_path(scope).ok().as_deref())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn install_writes_agents_md() {
        let dir = TempDir::new().unwrap();
        let scope = Scope::Local(dir.path().to_path_buf());
        Codex
            .install_prompt(&scope, &InstallOpts::default())
            .unwrap();
        let p = dir.path().join("AGENTS.md");
        assert!(p.exists());
        let contents = std::fs::read_to_string(&p).unwrap();
        assert!(contents.contains("ast-outline:begin"));
    }
}
