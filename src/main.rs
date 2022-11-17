use anyhow::Result;
use lapce_plugin::{
    psp_types::{
        lsp_types::{
            request::Initialize,  
            DocumentFilter, 
            DocumentSelector,
            InitializeParams, 
            MessageType, 
        },
        Request,
    },
    LapcePlugin, 
    PLUGIN_RPC,
    register_plugin, 
};
use serde_json::Value;
mod init;

#[derive(Default)]
struct State {}

register_plugin!(State);

fn initialize(params: InitializeParams) -> Result<()> {
    let document_selector: DocumentSelector = vec![
        DocumentFilter {
            language: Some("csharp".to_string()),
            pattern: Some("*.csproj".to_string()),
            scheme: None,
        }
    ];

    // Parse the config
    let (uri, server_args) = init::server_params_from_config(&params);
    
    let uri = if uri.is_none() { init::server_uri_default() } else { uri };
    
    if let Some(server_uri) = uri {
        PLUGIN_RPC.start_lsp(
            server_uri,
            server_args,
            document_selector,
            params.initialization_options,
        );
    } else {
        PLUGIN_RPC.stderr("Could not find server executable");
    }

    Ok(())
}

impl LapcePlugin for State {
    fn handle_request(&mut self, _id: u64, method: String, params: Value) {
        #[allow(clippy::single_match)]
        match method.as_str() {
            Initialize::METHOD => {
                let params: InitializeParams = serde_json::from_value(params).unwrap();
                if let Err(e) = initialize(params) {
                    PLUGIN_RPC.window_show_message(MessageType::ERROR, format!("plugin returned with error: {e}"))
                }
            }
            _ => {}
        }
    }
}
