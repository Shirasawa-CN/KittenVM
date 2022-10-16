use anyhow::Result;
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

impl<T> Stack<T> {
    pub fn pop(&mut self) -> Result<std::option::Option<T>, Error> {
        if self.value.is_empty() {
            return Err(std::fmt::Error);
        }
        Ok(self.value.pop())
    }
    pub fn peek(&mut self) -> Result<std::option::Option<&T>, Error> {
        if self.value.is_empty() {
            return Err(std::fmt::Error);
        }
        let index = self.value.len();
        Ok(self.value.get(index))
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

impl<T: std::ops::Add<Output = T> + 'static + Clone> DynamicMemory<T>
where
    T: Add<T>,
{
    fn free(&mut self) -> Result<(), Error> {
        for i in self.gc.position.iter() {
            if *i as usize > self.stack.value.len() {
                return Err(std::fmt::Error);
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
        self.gc.position.push(add_position);
        Ok(())
    }
    fn mov(&mut self, value: T, position: usize) -> Result<(), Error> {
        if position > self.stack.value.len() {
            return Err(std::fmt::Error);
        }
        Ok(self.stack.value[position] = value)
    }
    fn add(&mut self, rs1: usize, rs2: usize, target: usize) {
        self.stack.value[target] = self.stack.value[rs1].clone() + self.stack.value[rs2].clone();
    }
}
