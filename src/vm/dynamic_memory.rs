use anyhow::{Result, Ok};
use std::ops::Add;
use std::{cell::Ref, fmt::Error};

const CONST_POOL_INIT_CAP: usize = 8;

#[derive(Clone)]
struct Stack<T> {
    value: Vec<T>,
}

impl<T> Default for Stack<T> {
    fn default() -> Self {
        Self {
            value: Vec::<T>::with_capacity(CONST_POOL_INIT_CAP),
        }
    }
}

impl<T: Clone> Stack<T> {
    pub fn pop(&mut self) -> Result<std::option::Option<T>,anyhow::Error> {
        if self.value.is_empty() {
            tracing::error!(r#"Stack.value.is_empty"#);
            return Err(anyhow::anyhow!("Stack.value.is_empty"));
        }
        Ok(self.value.pop())
    }
    pub fn peek(&mut self) -> Result<std::option::Option<T>,anyhow::Error> {
        if self.value.is_empty() {
            tracing::error!(r#"Stack.value.is_empty"#);
            return Err(anyhow::anyhow!("Stack.value.is_empty"));
        }
        let index = self.value.len();
        Ok(Some(self.value[index].clone()))
    }
    pub fn push(&mut self, add_data: T) {
        self.value.push(add_data)
    }
}

struct Gc {
    position: Vec<usize>,
}

pub struct DynamicMemory<T> {
    stack: Stack<T>,
    gc: Gc,
}

impl<
        T:  'static
            + Clone
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Shl<Output = T>
            + std::ops::Shr<Output = T>
            + std::ops::BitXor<Output = T>
            + std::ops::BitOr<Output = T>
            + std::ops::BitAnd<Output = T>,
    > DynamicMemory<T>
where
    T: Add<T>,
{
    fn free(&mut self) -> Result<(), anyhow::Error> {
        for i in self.gc.position.iter() {
            if *i as usize > self.stack.value.len() {
                tracing::error!("The destination address is not in the allocated memory range. Traget:{}", i);
                return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
            }
            self.stack.value.remove(*i);
        }
        let i = self.gc.position.len() as usize;
        for x in i..0 {
            self.gc.position.remove(x);
        }
        Ok(())
    }
    fn add_gc(&mut self, add_position: usize) -> Result<()> {
        tracing::info!("add_gc add_position:{} Success",&add_position);
        self.gc.position.push(add_position);
        Ok(())
    }
    fn mov(&mut self, value: T, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = value;
        Ok(())
    }
    fn add(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() + self.stack.value[rs2].clone();
        Ok(())
    }
    fn sud(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() - self.stack.value[rs2].clone();
        Ok(())
    }
    fn mul(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() * self.stack.value[rs2].clone();
        Ok(())
    }
    fn div(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() / self.stack.value[rs2].clone();
        Ok(())
    }
    fn sll(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() << self.stack.value[rs2].clone();
        Ok(())
    }
    fn sra(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() >> self.stack.value[rs2].clone();
        Ok(())
    }
    fn xor(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() ^ self.stack.value[rs2].clone();
        Ok(())
    }
    fn or(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() | self.stack.value[rs2].clone();
        Ok(())
    }
    fn and(&mut self, rs1: usize, rs2: usize, target: usize) -> Result<(), anyhow::Error> {
        if target > self.stack.value.len() {
            tracing::error!("The destination address is not in the allocated memory range. Traget:{}", &target);
            return Err(anyhow::anyhow!("The destination address is not in the allocated memory range"));
        }
        self.stack.value[target] = self.stack.value[rs1].clone() & self.stack.value[rs2].clone();
        Ok(())
    }
}
