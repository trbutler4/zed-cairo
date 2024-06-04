use zed_extension_api as zed;

struct CairoExtension {}

const SCARB_BINARY: &str = "scarb";

impl zed::Extension for CairoExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let (command, args) = match worktree.which(SCARB_BINARY) {
            Some(command) => (
                command,
                vec!["cairo-language-server".into(), "--stdio".into()],
            ),
            None => {
                Err("scarb not found in PATH")?;
                ("".into(), vec![])
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
