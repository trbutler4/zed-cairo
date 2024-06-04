use std::{env, fs};

use zed::settings::LspSettings;
use zed::Worktree;
use zed_extension_api as zed;

const SERVER_PATH: &str = "node_modules/cairo-ls/out/server.js";
const CAIRO_LS_PACKAGE: &str = "cairo-ls";

struct CairoExtension {
    found_cairo_ls: bool,
}

impl CairoExtension {
    fn cairo_ls_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn cairo_ls_script_path(&mut self, id: &zed::LanguageServerId) -> zed::Result<String> {
        let cairo_ls_exists = self.cairo_ls_exists();
        if self.found_cairo_ls && cairo_ls_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(CAIRO_LS_PACKAGE)?;

        if !cairo_ls_exists
            || zed::npm_package_installed_version(CAIRO_LS_PACKAGE)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let result = zed::npm_install_package(CAIRO_LS_PACKAGE, &version);
            match result {
                Ok(_) => {
                    if !self.cairo_ls_exists() {
                        Err(format!("installed package '{CAIRO_LS_PACKAGE}' did not contain expected path '{SERVER_PATH}'"))?;
                    }
                }
                Err(e) => {
                    if !self.cairo_ls_exists() {
                        Err(e)?;
                    }
                }
            }
        }

        self.found_cairo_ls = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for CairoExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            found_cairo_ls: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let (command, args) = match worktree.which("scarb") {
            Some(command) => (
                command,
                vec!["cairo-language-server".into(), "--stdio".into()],
            ),
            None => {
                let script_path = self.cairo_ls_script_path(language_server_id)?;
                let current_dir = env::current_dir().unwrap();
                let ls_script = current_dir.join(&script_path).to_string_lossy().to_string();
                let command = zed::node_binary_path()?;
                let args = vec![ls_script.into(), "--stdio".into()];
                (command, args)
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
