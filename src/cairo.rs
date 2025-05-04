use std::{env, fs, path::PathBuf};
use zed_extension_api::{self as zed, Result};

struct CairoExtension;

// ---------- platform-specific defaults --------------------------------
#[cfg(target_os = "macos")]
fn default_scarb_cache_dir() -> Option<PathBuf> {
    dirs::home_dir().map(|h| h.join("Library").join("Caches").join("com.swmansion.scarb"))
}

#[cfg(target_os = "linux")]
fn default_scarb_cache_dir() -> Option<PathBuf> {
    env::var("XDG_CACHE_HOME")
        .ok()
        .map(PathBuf::from)
        .or_else(|| dirs::home_dir().map(|h| h.join(".cache")))
        .map(|p| p.join("scarb"))
}

#[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
fn default_scarb_cache_dir() -> Option<PathBuf> {
    None
}

#[cfg(target_os = "windows")]
fn default_scarb_cache_dir() -> Option<PathBuf> {
    env::var("LocalAppData").ok().map(|p| {
        PathBuf::from(p)
            .join("swmansion")
            .join("scarb")
            .join("cache")
    })
}

fn find_corelib_dir() -> Option<PathBuf> {
    eprintln!("FINDING CAIRO CORELIB");
    // 1. Explicit override
    if let Ok(p) = env::var("CAIRO_CORELIB_DIR") {
        return Some(PathBuf::from(p));
    }

    // 2. Locate Scarb cache root
    let cache_root = env::var("SCARB_CACHE")
        .ok()
        .map(PathBuf::from)
        .or_else(default_scarb_cache_dir)?;

    eprintln!("cache root: {:?}", cache_root);

    // 3. Look for registry/std/<version>/core/…
    let std_registry = cache_root.join("registry").join("std");
    eprintln!("std registry: {:?}", std_registry);

    let latest_version = fs::read_dir(&std_registry)
        .ok()?
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().ok().map(|t| t.is_dir()).unwrap_or(false))
        .max_by_key(|e| e.file_name())? // pick “v2.12.0” over “v2.11.4”
        .path(); // e.g. “…/v2.11.4”

    eprintln!("latest version: {:?}", latest_version);

    Some(latest_version)
}

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

    fn language_server_workspace_configuration(
        &mut self,
        _: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let corelib = PathBuf::from(
            "/Users/francos/Library/Caches/com.swmansion.scarb/registry/std/v2.11.4/",
        );

        Ok(Some(serde_json::json!({
            "settings": {
                "cairo1": {
                    "corelibPath": corelib,
                    "enableProcMacros": true,
                    "enableLinter": true,
                    "traceMacroDiagnostics": false
                }
            }
        })))
    }

    fn language_server_additional_workspace_configuration(
        &mut self,
        _ls: &zed::LanguageServerId,
        _target_ls: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let corelib = PathBuf::from(
            "/Users/francos/Library/Caches/com.swmansion.scarb/registry/std/v2.11.4/",
        );
        Ok(Some(serde_json::json!([
            corelib, // cairo1.corelibPath
            false,   // cairo1.traceMacroDiagnostics
            true,    // cairo1.enableProcMacros
            true     // cairo1.enableLinter
        ])))
    }
}

zed::register_extension!(CairoExtension);
