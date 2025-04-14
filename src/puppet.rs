use serde_json::Value;
use zed_extension_api::{self as zed, Result, serde_json, settings::LspSettings};

const PUPPET_LANGUAGE_SERVER_NAME: &str = "puppet-languageserver";

fn merge(a: &mut Value, b: &Value) {
    match (a, b) {
        (Value::Object(a), Value::Object(b)) => {
            for (k, v) in b {
                merge(a.entry(k.clone()).or_insert(Value::Null), v);
            }
        }
        (a, b) => *a = b.clone(),
    }
}

struct PuppetExtension;

impl PuppetExtension {
    fn server_path(&self, worktree: &zed::Worktree) -> Result<String> {
        // Check for custom binary path in settings first
        let binary_settings = LspSettings::for_worktree(PUPPET_LANGUAGE_SERVER_NAME, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        if let Some(path) = binary_settings.and_then(|settings| settings.path) {
            return Ok(path);
        }

        // Fall back to searching in PATH
        if let Some(path) = worktree.which(PUPPET_LANGUAGE_SERVER_NAME) {
            return Ok(path);
        }

        Err(format!(
            "Puppet language server '{}' not found. Please install puppet-editor-services or configure a custom path in settings.",
            PUPPET_LANGUAGE_SERVER_NAME
        ))
    }

    fn server_args(&self, worktree: &zed::Worktree) -> Vec<String> {
        let binary_settings = LspSettings::for_worktree(PUPPET_LANGUAGE_SERVER_NAME, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        binary_settings
            .and_then(|settings| settings.arguments)
            .unwrap_or_else(|| vec!["--stdio".to_string(), "--no-cache".to_string()])
    }

    fn get_puppet_config(&self, worktree: &zed::Worktree) -> (String, String, String, String) {
        // Try to get Puppet configuration using puppet config print
        if let Some(puppet_path) = worktree.which("puppet") {
            if let Ok(output) = std::process::Command::new(puppet_path)
                .args(&[
                    "config",
                    "print",
                    "confdir",
                    "vardir",
                    "environment",
                    "modulepath",
                    "--render-as",
                    "json",
                ])
                .output()
            {
                if output.status.success() {
                    if let Ok(config_str) = String::from_utf8(output.stdout) {
                        if let Ok(config) = serde_json::from_str::<serde_json::Value>(&config_str) {
                            let confdir = config["confdir"]
                                .as_str()
                                .unwrap_or("/etc/puppetlabs/puppet")
                                .to_string();
                            let vardir = config["vardir"]
                                .as_str()
                                .unwrap_or("/opt/puppetlabs/puppet/cache")
                                .to_string();
                            let environment = config["environment"]
                                .as_str()
                                .unwrap_or("production")
                                .to_string();

                            // Build module path with project-local paths first
                            let base_modulepath = config["modulepath"].as_str()
                                .unwrap_or("/etc/puppetlabs/code/environments/production/modules:/opt/puppetlabs/puppet/modules");
                            let module_path =
                                format!("./modules:./site-modules:{}", base_modulepath);

                            return (confdir, vardir, environment, module_path);
                        }
                    }
                }
            }
        }

        // Fallback to default Puppetlabs paths if puppet command fails
        (
            "/etc/puppetlabs/puppet".to_string(),
            "/opt/puppetlabs/puppet/cache".to_string(),
            "production".to_string(),
            "./modules:./site-modules:/etc/puppetlabs/code/environments/production/modules:/opt/puppetlabs/puppet/modules".to_string()
        )
    }
}

impl zed::Extension for PuppetExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_path(worktree)?;
        let args = self.server_args(worktree);

        Ok(zed::Command {
            command: server_path,
            args,
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let (confdir, vardir, environment, module_path) = self.get_puppet_config(worktree);

        // Build settings using Puppet's actual configuration
        let mut final_settings = serde_json::json!({
            "puppet": {
                "modulePath": module_path,
                "confdir": confdir,
                "vardir": vardir,
                "environment": environment,
                "editorService": {
                    "enable": true,
                    "puppet": {
                        "confdir": confdir,
                        "environment": environment,
                        "modulePath": module_path,
                        "vardir": vardir
                    },
                    "hover": {
                        "showMetadataInfo": true,
                        "enable": true
                    },
                    "validation": {
                        "enableSyntax": true,
                        "enableSemantic": true
                    },
                    "completion": {
                        "enable": true
                    },
                    "formatOnType": {
                        "enable": true
                    }
                }
            }
        });

        // Merge with user settings
        let zed_lsp_settings = LspSettings::for_worktree(PUPPET_LANGUAGE_SERVER_NAME, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        merge(&mut final_settings, &zed_lsp_settings);

        Ok(Some(final_settings))
    }

    fn language_server_initialization_options(
        &mut self,
        _server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        // Keep initialization minimal - most config goes in workspace configuration
        Ok(Some(serde_json::json!({
            "provideFormatter": true,
            "provideHover": true,
            "provideDefinition": true,
            "provideCompletion": true
        })))
    }
}

zed::register_extension!(PuppetExtension);
