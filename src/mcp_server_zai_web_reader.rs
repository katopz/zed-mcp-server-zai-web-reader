use schemars::JsonSchema;
use serde::Deserialize;
use zed::settings::ContextServerSettings;
use zed_extension_api::{
    self as zed, serde_json, Command, ContextServerConfiguration, ContextServerId, Project, Result,
};

const PACKAGE_NAME: &str = "supergateway";
const PACKAGE_VERSION: &str = "3.4.3";
const MCP_SERVER_URL: &str = "https://api.z.ai/api/mcp/web_reader/mcp";

#[derive(Debug, Deserialize, JsonSchema)]
struct ZaiWebReaderMcpExtensionSettings {
    /// The API key for z.ai web reader service.
    #[serde(default)]
    zai_api_key: Option<String>,
}

struct ZaiWebReaderMcpExtension;

impl zed::Extension for ZaiWebReaderMcpExtension {
    fn new() -> Self {
        Self
    }

    fn context_server_command(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Command> {
        if zed::npm_package_installed_version(PACKAGE_NAME)?.is_none() {
            zed::npm_install_package(PACKAGE_NAME, PACKAGE_VERSION)?;
        }

        let settings = ContextServerSettings::for_project("mcp-server-zai-web-reader", project)?;

        let settings_struct: ZaiWebReaderMcpExtensionSettings = match settings.settings {
            Some(value) => serde_json::from_value(value).map_err(|e| format!("{e}"))?,
            None => ZaiWebReaderMcpExtensionSettings { zai_api_key: None },
        };

        let server_path = format!(
            "{}/node_modules/supergateway/dist/index.js",
            std::env::current_dir()
                .map_err(|e| format!("{e}"))?
                .to_string_lossy()
        );

        let mut args = vec![
            server_path,
            "--streamableHttp".to_string(),
            MCP_SERVER_URL.to_string(),
            "--logLevel".to_string(),
            "info".to_string(),
        ];

        if let Some(api_key) = settings_struct.zai_api_key {
            if !api_key.is_empty() {
                args.push("--oauth2Bearer".to_string());
                args.push(api_key);
            }
        }

        Ok(Command {
            command: zed::node_binary_path()?,
            args,
            env: Default::default(),
        })
    }

    fn context_server_configuration(
        &mut self,
        _context_server_id: &ContextServerId,
        project: &Project,
    ) -> Result<Option<ContextServerConfiguration>> {
        let installation_instructions =
            include_str!("../configuration/installation_instructions.md").to_string();

        let settings = ContextServerSettings::for_project("mcp-server-zai-web-reader", project);

        let mut default_settings =
            include_str!("../configuration/default_settings.jsonc").to_string();

        if let Ok(user_settings) = settings {
            if let Some(settings_value) = user_settings.settings {
                if let Ok(reader_settings) =
                    serde_json::from_value::<ZaiWebReaderMcpExtensionSettings>(settings_value)
                {
                    match reader_settings.zai_api_key {
                        Some(zai_api_key) => {
                            default_settings = default_settings
                                .replace("\"YOUR_ZAI_API_KEY\"", &format!("\"{}\"", zai_api_key));
                        }
                        None => {
                            default_settings =
                                default_settings.replace("\"YOUR_ZAI_API_KEY\"", "\"\"");
                        }
                    }
                }
            }
        }

        let settings_schema =
            serde_json::to_string(&schemars::schema_for!(ZaiWebReaderMcpExtensionSettings))
                .map_err(|e| format!("{e}"))?;

        Ok(Some(ContextServerConfiguration {
            installation_instructions,
            default_settings,
            settings_schema,
        }))
    }
}

zed::register_extension!(ZaiWebReaderMcpExtension);
