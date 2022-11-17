use lapce_plugin::{
    psp_types::lsp_types::{ InitializeParams, Url },
    VoltEnvironment, 
};

pub fn server_params_from_config(params: &InitializeParams) -> (Option<Url>, Vec<String>) {
    let lsp = params
        .initialization_options
        .as_ref()
        .and_then(|options| options.get("lsp"));

    if lsp.is_none() {
        return (None, vec![]);
    }
    let lsp = lsp.unwrap();

    let mut server_args = vec![];
    if let Some(args) = lsp
        .get("serverArgs") 
        .and_then(|args| args.as_array()) {
            for arg in args {
                if let Some(arg) = arg.as_str() {
                    server_args.push(arg.to_string());
                }
            }
        }

    let server_uri = match lsp
        .get("serverPath")
        .and_then(|server_path| server_path.as_str()) {
            Some(server_path) => Some(
                Url::parse(
                    &format!("urn:{}", server_path)
                ).unwrap()
            ),
            None => None,
        };

    (server_uri, server_args)
}

pub fn server_uri_default() -> Option<Url> {
    // Architecture check
    let arch = match VoltEnvironment::architecture().as_deref() {
        Ok("x86_64") => "x64",
        Ok("aarch64") => "aarch64",
        _ => return None,
    };

    // OS check
    let os = match VoltEnvironment::operating_system().as_deref() {
        Ok("macos") => "macos",
        Ok("linux") => "linux",
        Ok("windows") => "windows",
        _ => return None,
    };

    // Download URL
    // let _ = format!("https://github.com/<name>/<project>/releases/download/<version>/{filename}");

    // see lapce_plugin::Http for available API to download files

    let location: String = format!("omnisharp-{os}-{arch}-net6.0");
    let filename = "OmniSharp";
    let filename = 
        if os == "windows" { 
            format!("{filename}.exe") 
        } else {
            filename.to_string() 
        };

    // Plugin working directory
    let volt_uri = VoltEnvironment::uri().unwrap();
    let server_uri = 
        Url::parse(&volt_uri).unwrap()
            .join(&location).unwrap()
            .join(&filename).unwrap();

    // if you want to use server from PATH
    // let server_uri = Url::parse(&format!("urn:{filename}"))?;

    // Available language IDs
    // https://github.com/lapce/lapce/blob/HEAD/lapce-proxy/src/buffer.rs#L173
    Some(server_uri)
}