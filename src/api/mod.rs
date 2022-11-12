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

pub struct API<T> {
    vm: machine::KittenVM<T>,
}

impl<
        T: 'static
            + Clone
            + NumOps
            + std::ops::BitAnd
            + std::ops::BitXor
            + std::ops::BitOr
            + std::ops::Shl
            + std::ops::Shr
            + std::str::FromStr,
    > API<T>
where
    <T as FromStr>::Err: Sync,
    <T as FromStr>::Err: std::error::Error,
    <T as FromStr>::Err: Send,
    Arc<T>: NumOps,
    Arc<T>: std::ops::BitAnd<Output = Arc<T>>,
    Arc<T>: std::ops::BitXor<Output = Arc<T>>,
    Arc<T>: std::ops::BitOr<Output = Arc<T>>,
    Arc<T>: std::ops::Shl<Output = Arc<T>>,
    Arc<T>: std::ops::Shr<Output = Arc<T>>,
{
    pub async fn file(_file_path: &Path) -> Result<()> {
        Ok(())
    }

    pub async fn stream(&mut self, code: String) -> Result<(), anyhow::Error> {
        machine::KittenVM::matcher(&mut self.vm, code).await
    }
}
