use anyhow::{Ok, Result};
use std::collections::HashMap;

//存储变量名和位置
#[derive(Default)]
pub struct ConstPool {
    Pool: HashMap<String, usize>,
}

const CONST_POOL_INIT_CAP: usize = 8;

impl ConstPool {
    pub fn new(&self) -> Self {
        Self {
            Pool: HashMap::with_capacity(CONST_POOL_INIT_CAP),
        }
    }
    pub fn add(&mut self, name: String, target: usize) -> Result<()> {
        self.Pool.insert(name, target);
        Ok(())
    }
    pub fn search(&self, name: String) -> Result<Option<&usize>, anyhow::Error> {
        let result = self.Pool.get(&name);
        if result.is_some() {
            Ok(result)
        } else {
            tracing::error!("Can't find {}", name);
            Err(anyhow::anyhow!("Can't find {}", name))
        }
    }
}
