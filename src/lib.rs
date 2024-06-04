use zed_extension_api as zed;

struct CairoExtension {}

impl zed::Extension for CairoExtension {
    // ...
    fn new() -> Self {
        CairoExtension {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let lsp_args = vec!["cairo-language-server".into(), "--stdio".into()];
        let (command, args) = match worktree.which("scarb") {
            Some(command) => (command, lsp_args),
            None => (
                "node".into(),
                vec![
                    "./cairo-ls/node_modules/cairo-ls/out/server.js".into(),
                    "--stdio".into(),
                ],
            ),
        };
        Ok(zed::Command {
            command,
            args,
            env: Default::default(),
        })
    }
}

zed::register_extension!(CairoExtension);
