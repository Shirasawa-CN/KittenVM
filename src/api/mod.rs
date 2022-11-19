use crate::vm::machine;
use anyhow::{Ok, Result};
use num_traits::NumOps;
use std::{path::Path, str::FromStr, sync::Arc};

enum SafeMode {
    Safe,
    Normal,
}

enum GCMode {
    SimpleGC,
    UnGC,
}
pub struct Mode {
    safe_mode: SafeMode,
    gc_mode: GCMode,
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            safe_mode: SafeMode::Normal,
            gc_mode: GCMode::UnGC,
        }
    }
}

pub struct API {
    vm: machine::KittenVM,
}

impl API {
    pub async fn file(_file_path: &Path) -> Result<()> {
        Ok(())
    }

    pub async fn stream(&mut self, code: String) -> Result<(), anyhow::Error> {
        machine::KittenVM::matcher(&mut self.vm, code).await
    }
}
