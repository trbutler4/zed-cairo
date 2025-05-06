use std::path::PathBuf;
use zed::serde_json;
use zed_extension_api::{self as zed, Result};

struct CairoExtension;

impl zed::Extension for CairoExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("scarb")
            .ok_or_else(|| "scarb must be installed via asdf".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["cairo-language-server".into(), "--stdio".into()],
            env: worktree.shell_env(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let out = zed::Command::new("scarb")
            .arg("cache")
            .arg("path")
            .output()?;

        if out.status != Some(0) {
            return Err(format!(
                "`scarb cache path` failed (exit {:?}): {}",
                out.status,
                String::from_utf8_lossy(&out.stderr)
            ));
        }

        let cache_root_str = std::str::from_utf8(&out.stdout)
            .map_err(|e| format!("Failed to parse stdout: {}", e))?
            .trim();

        let corelib = PathBuf::from(cache_root_str);

        let cfg = serde_json::json!({
            "cairo1.corelibPath": corelib,
            "cairo1.traceMacroDiagnostics": false,
            "cairo1.enableProcMacros": true,
            "cairo1.enableLinter": true
        });
        Ok(Some(cfg))
    }

    fn language_server_initialization_options(
        &mut self,
        _id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        eprintln!("Cairo-ext: LS initialization_options called");
        Ok(Some(serde_json::json!({
            // Tell Zed to mask this capability entirely
            "disableCapabilities": ["documentHighlightProvider"]
        })))
    }
}

zed::register_extension!(CairoExtension);
