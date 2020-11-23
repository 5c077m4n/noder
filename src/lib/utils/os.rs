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
    let os_name = get_os_name();
    if os_name.is_none() {
        return None;
    }
    let os_arch = get_os_arch();
    if os_arch.is_none() {
        return None;
    }
    let ext = match os_name.unwrap() {
        "linux" | "darwin" => Some(".tar.gz"),
        "win" => Some(".zip"),
        _ => None,
    };

    Some(format!(
        "node-{version}-{os_name}-{os_arch}{ext}",
        version = version,
        os_name = os_name.unwrap(),
        os_arch = os_arch.unwrap(),
        ext = ext.unwrap(),
    ))
}
