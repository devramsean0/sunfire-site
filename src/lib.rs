pub fn normalize_http_path(path: &str) -> String {
    let has_extension = path.rsplit('/')
    .next()
    .and_then(|segment| segment.split('.').nth(1))
    .is_some();

    let is_api = path.starts_with("/api/");
    
    if is_api {
        return path.to_string()
    }

    if !has_extension {
        if path.ends_with('/') {
            format!("{}index.html", path)
        } else {
            format!("{}/index.html", path)
        }
    } else {
        path.to_string()
    }
}

pub fn build_asset_path(path: &str) -> String {
    format!("build{}", path)
}