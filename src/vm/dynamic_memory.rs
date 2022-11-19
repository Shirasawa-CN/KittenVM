use std::{collections::HashMap, sync::Arc, usize};

use anyhow::{Ok, Result};

#[derive(Clone, Default)]
pub struct Stack {
    pub value: Vec<Arc<super::value_type::Type>>,
    pub pool: HashMap<String, usize>,
}

impl Stack {
    pub fn pop(
        &mut self,
    ) -> Result<std::option::Option<std::sync::Arc<super::value_type::Type>>, anyhow::Error> {
        if self.value.is_empty() {
            tracing::error!("Stack.value.is_empty");
            return Err(anyhow::anyhow!("Stack.value.is_empty"));
        }
        Ok(self.value.pop())
    }
    pub fn peek(
        &mut self,
    ) -> Result<std::option::Option<std::sync::Arc<super::value_type::Type>>, anyhow::Error> {
        if self.value.is_empty() {
            tracing::error!("Stack.value.is_empty");
            return Err(anyhow::anyhow!("Stack.value.is_empty"));
        }
        let index = self.value.len();
        Ok(Some(self.value[index].clone()))
    }
    pub fn push(&mut self, add_data: super::value_type::Type) {
        self.value.push(Arc::new(add_data.clone()));
    }
}

#[derive(Default)]
pub struct Gc {
    pub position: Vec<usize>,
}

#[derive(Default)]
pub struct DynamicMemory {
    pub stack: Stack,
    pub gc: Gc,
}

impl DynamicMemory {
    pub async fn add(
        &mut self,
        rs1: usize,
        rs2: usize,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] + *self.stack.value[rs2]);
        Ok(())
    }
    pub async fn add_gc(&mut self, add_position: usize) -> Result<()> {
        tracing::info!("add_gc add_position:{} Success", &add_position);
        self.gc.position.push(add_position);
        Ok(())
    }
    pub async fn and(
        &mut self,
        rs1: usize,
        rs2: usize,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] & *self.stack.value[rs2]);
        Ok(())
    }
    pub async fn div(
        &mut self,
        rs1: usize,
        rs2: usize,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] / *self.stack.value[rs2]);
        Ok(())
    }
    /*
    动态内存池指令
    */
    pub fn free(&mut self) -> Result<(), anyhow::Error> {
        for i in self.gc.position.iter() {
            if *i as usize > self.stack.value.len() {
                tracing::error!(
                    "The destination address is not in the allocated memory range. Traget:{}",
                    i
                );
                return Err(anyhow::anyhow!(
                    "The destination address is not in the allocated memory range"
                ));
            }
            drop(&self.stack.value[*i]);
            self.stack.value.remove(*i);
        }
        let i = self.gc.position.len() as usize;
        for x in i..0 {
            self.gc.position.remove(x);
        }
        Ok(())
    }
    pub async fn mov(
        &mut self,
        value: super::value_type::Type,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(value);
        Ok(())
    }
    pub async fn mul(
        &mut self,
        rs1: usize,
        rs2: usize,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] * *self.stack.value[rs2]);
        Ok(())
    }
    pub async fn new_mem(&mut self, rs: super::value_type::Type) -> usize {
        self.stack.value.push(Arc::new(rs));
        self.stack.value.len()
    }
    pub async fn or(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] | *self.stack.value[rs2]);
        Ok(())
    }
    pub async fn sll(
        &mut self,
        rs1: usize,
        rs2: usize,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] << *self.stack.value[rs2]);
        Ok(())
    }
    pub async fn sra(
        &mut self,
        rs1: usize,
        rs2: usize,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] >> *self.stack.value[rs2]);
        Ok(())
    }
    pub async fn sud(
        &mut self,
        rs1: usize,
        rs2: usize,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] - *self.stack.value[rs2]);
        Ok(())
    }
    pub async fn xor(
        &mut self,
        rs1: usize,
        rs2: usize,
        target: usize,
    ) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!(
                "The destination address is not in the allocated memory range. Traget:{}",
                &target
            );
            return Err(anyhow::anyhow!(
                "The destination address is not in the allocated memory range"
            ));
        }
        self.stack.value[target] = Arc::new(*self.stack.value[rs1] ^ *self.stack.value[rs2]);
        Ok(())
    }
}
