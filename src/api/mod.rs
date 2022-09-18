use anyhow::Result;

#[cfg(target_os = "windows")]
fn build_binary(file_path: String) -> Result<String> {
    Ok(file_path)
}

#[allow(dead_code)]
#[cfg(target_os = "linux")]
fn build_binary(file_path: String) -> Result<String> {
    Ok(file_path)
}
