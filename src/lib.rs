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
        let lsp_args = vec!["--stdio".into()];
        let (command, args) = match worktree.which("scarb") {
            Some(command) => {
                println!("Using scarb from {:?}", command);
                (command, lsp_args)
            }
            None => {
                println!("scarb not found in path");
                ("$HOME/.local/bin/scarb".into(), lsp_args)
            }
        };
        Ok(zed::Command {
            command,
            args,
            env: Default::default(),
        })
    }
}

zed::register_extension!(CairoExtension);
