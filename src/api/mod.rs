use anyhow::{Ok, Result};
use std::path::Path;

enum SafeMode {
    Safe,
    Normal,
}

enum GCMode{
    SimpleGC,
    UnGC,
}
pub struct Mode{
    safe_mode: SafeMode,
    gc_mode: GCMode,
}


pub fn stream(code: String) -> Result<()> {
    Ok(())
}

pub fn file(_file_path: &Path) -> Result<()> {
    Ok(())
}
