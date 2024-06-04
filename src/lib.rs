use std::{env, fs};

use zed::settings::LspSettings;
use zed::Worktree;
use zed_extension_api as zed;

const SERVER_PATH: &str = "cairo-ls/node_modules/cairo-ls/server.js";
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
                        Err(format!("installed package `{CAIRO_LS_PACKAGE}` did not contain expected path '{SERVER_PATH}'"))?;
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
        let (command, args) = match worktree.which("TODO") {
            Some(command) => (command, vec![]),
            None => {
                let server_path = self.cairo_ls_script_path(language_server_id)?;
                let node = env::var("NODE").unwrap_or_else(|_| "node".to_string());
                (node, vec![server_path, "--stdio".into()])
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
