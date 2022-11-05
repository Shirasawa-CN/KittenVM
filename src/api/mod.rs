use crate::vm::machine;
use anyhow::{Ok, Result};
use std::{path::Path, str::FromStr};

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
            + std::str::FromStr
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Shl<Output = T>
            + std::ops::Shr<Output = T>
            + std::ops::BitXor<Output = T>
            + std::ops::BitOr<Output = T>
            + std::ops::BitAnd<Output = T>,
    > API<T>
{
    pub fn file(_file_path: &Path) -> Result<()> {
        Ok(())
    }

    pub fn stream(&mut self, code: String) -> Result<(), anyhow::Error>
    where
        <T as FromStr>::Err: Sync,
        <T as FromStr>::Err: std::error::Error,
        <T as FromStr>::Err: Send,
    {
        let result = machine::KittenVM::matcher(&mut self.vm, code);
        result
    }
}
