use std::path::PathBuf;

use super::paths;
use super::{common, Change, Detection, InstallOpts, Installer, Scope, Status};
use crate::prompt::AGENT_PROMPT;

pub struct Copilot;

impl Copilot {
    fn prompt_path(&self, scope: &Scope) -> Result<PathBuf, String> {
        match scope {
            Scope::Local(root) => Ok(root.join(".github/copilot-instructions.md")),
            Scope::Global => paths::under_home(".copilot/copilot-instructions.md"),
        }
    }
}

impl Installer for Copilot {
    fn name(&self) -> &'static str {
        "copilot"
    }

    fn detect(&self, scope: &Scope) -> Detection {
        let p = match self.prompt_path(scope) {
            Ok(p) => p,
            Err(_) => {
                return Detection {
                    present: false,
                    config_root: None,
                }
            }
        };
        let root = p.parent().map(|p| p.to_path_buf());
        Detection {
            present: root.as_ref().map(|r| r.exists()).unwrap_or(false),
            config_root: root,
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
    fn install_writes_copilot_instructions() {
        let dir = TempDir::new().unwrap();
        let scope = Scope::Local(dir.path().to_path_buf());
        Copilot
            .install_prompt(&scope, &InstallOpts::default())
            .unwrap();
        let p = dir.path().join(".github/copilot-instructions.md");
        assert!(p.exists());
    }
}
