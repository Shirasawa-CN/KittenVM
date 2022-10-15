use anyhow::{Ok, Result};
use std::path::Path;

pub enum Gc {
    SimpleGC,
    NGC,
}
pub enum Safe{
    Safe,
    Unsafe,
}
pub struct Mode{
    gc: Gc,
    safe: Safe,
}

pub fn file(_file_path: &Path) -> Result<()> {
    Ok(())
}
