use zed_extension_api::{self as zed, Result};

struct FlutterExtension;

impl zed::Extension for FlutterExtension {
    fn new() -> Self {
        Self
    }

    fn get_dap_binary(
        &mut self,
        _adapter_name: String,
        _config: zed::DebugTaskDefinition,
        _user_provided_debug_adapter_path: Option<String>,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        // 1. Find the 'flutter' executable in the user's path
        let flutter_path = worktree
            .which("flutter")
            .ok_or_else(|| "Flutter executable not found in PATH.".to_string())?;

        // 2. Return the command to start the DAP server
        // The magic command is: `flutter debug_adapter`
        Ok(zed::Command {
            command: flutter_path,
            args: vec!["debug_adapter".to_string()],
            env: Default::default(),
        })
    }

    fn dap_request_kind(
        &mut self,
        _adapter_name: String,
        _config: zed::DebugTaskDefinition,
    ) -> Result<zed::StartDebuggingRequestArgumentsRequest> {
        // Flutter supports both launching a new app and attaching to a running one.
        // We usually default to Launch.
        Ok(zed::StartDebuggingRequestArgumentsRequest::Launch)
    }

    fn dap_config_to_scenario(
        &mut self,
        config: zed::DebugTaskDefinition,
    ) -> Result<zed::DebugScenario> {
        // This helper maps the JSON config from tasks.json to the structure
        // the DAP server expects.
        
        // Grab the 'program' (e.g. lib/main.dart)
        let program = config
            .get("program")
            .and_then(|v| v.as_str())
            .ok_or("The 'program' field is required (e.g., lib/main.dart)")?;

        // Grab any CLI args (e.g. ["--flavor", "dev"])
        let args = config
            .get("args")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();
            
        // Grab toolArgs (e.g. device id "-d iphone15")
        // We put these into "toolArgs" because that's what Flutter DAP expects
        let tool_args = config
            .get("toolArgs")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default();

        let mut arguments = serde_json::json!({
            "program": program,
            "args": args,
            "toolArgs": tool_args,
            "noDebug": false,
        });

        Ok(zed::DebugScenario {
            command: "flutter".to_string(), // This is just a label for the scenario
            args: arguments.as_object().unwrap().clone(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(FlutterExtension);