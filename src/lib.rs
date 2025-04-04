use std::path::PathBuf;
use std::{fs, process::Command as StdCommand};

use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

const REPO_URL: &str = "https://github.com/puppetlabs/puppet-editor-services.git";
const INSTALL_DIR: &str = ".zed-puppet-lsp";
const SERVER_SCRIPT: &str = "puppet-languageserver";

struct PuppetExtension {
    did_setup: bool,
}

impl PuppetExtension {
    fn lsp_dir() -> PathBuf {
        dirs::home_dir()
            .expect("Could not find home dir")
            .join(INSTALL_DIR)
    }

    fn server_script_path() -> PathBuf {
        Self::lsp_dir().join(SERVER_SCRIPT)
    }

    fn server_exists() -> bool {
        let path = Self::server_script_path();
        fs::metadata(&path).map_or(false, |m| m.is_file())
    }

    fn setup(&self) -> Result<()> {
        let path = Self::lsp_dir();

        if !path.exists() {
            // Clone the repo
            let status = StdCommand::new("git")
                .args(["clone", REPO_URL, path.to_str().unwrap()])
                .status()
                .map_err(|e| e.to_string())?;

            if !status.success() {
                return Err("Échec du clonage de puppet-editor-services".into());
            }
        }

        // Run `bundle install`
        let status = StdCommand::new("bundle")
            .arg("install")
            .current_dir(&path)
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err("Échec de l'installation des gems (bundle install)".into());
        }

        // Run `rake cache:setup`
        let status = StdCommand::new("rake")
            .args(["cache:setup"])
            .current_dir(&path)
            .status()
            .map_err(|e| e.to_string())?;

        if !status.success() {
            return Err(
                "Échec de l'installation des dépendances locales (rake cache:setup)".into(),
            );
        }

        Ok(())
    }
}

impl zed::Extension for PuppetExtension {
    fn new() -> Self {
        Self { did_setup: false }
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if !self.did_setup || !Self::server_exists() {
            self.setup()?;
            self.did_setup = true;
        }

        let server_path = Self::server_script_path();

        Ok(zed::Command {
            command: server_path.to_str().unwrap().to_string(),
            args: vec!["--stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(PuppetExtension);
