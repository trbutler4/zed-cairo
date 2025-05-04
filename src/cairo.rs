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
        // Simple logging to debug LSP startup
        eprintln!("Cairo LSP: Attempting to locate scarb");

        let path = worktree
            .which("scarb")
            .ok_or_else(|| "scarb must be installed via asdf".to_string())?;

        // Log that we found scarb
        eprintln!("Cairo LSP: Found scarb at: {:?}", path);

        // Log the command being executed
        eprintln!(
            "Cairo LSP: Executing command: {:?} cairo-language-server --stdio",
            path
        );
        Ok(zed::Command {
            command: path,
            args: vec!["cairo-language-server".into(), "--stdio".into()],
            env: worktree.shell_env(),
        })
    }
}

zed::register_extension!(CairoExtension);
