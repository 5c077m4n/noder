pub fn get_os_name() -> Option<&'static str> {
    match std::env::consts::OS {
        "linux" => Some("linux"),
        "macos" => Some("darwin"),
        "windows" => Some("win"),
        _ => None,
    }
}

pub fn get_os_arch() -> Option<&'static str> {
    match std::env::consts::ARCH {
        "x86" => Some("x86"),
        "x86_64" => Some("x64"),
        _ => None,
    }
}

pub fn get_os_node_file_name(version: &str) -> Option<String> {
    let os_name = get_os_name().unwrap_or_else(|| panic!("Sorry, your OS is not supported"));
    let os_arch = get_os_arch().unwrap_or_else(|| panic!("Sorry, your OS arch is not supported"));

    let ext = match os_name {
        "linux" | "darwin" => Some(".tar.gz"),
        "win" => Some(".zip"),
        _ => None,
    };

    Some(format!(
        "node-{version}-{os_name}-{os_arch}{ext}",
        version = version,
        os_name = os_name,
        os_arch = os_arch,
        ext = ext.unwrap(),
    ))
}
