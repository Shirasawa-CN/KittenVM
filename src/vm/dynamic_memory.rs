use std::fmt::Error;

use anyhow::Result;
const CONST_POOL_INIT_CAP: usize = 8;

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

#[derive(Default)]
struct Gc {
    position: Vec<usize>,
}

pub struct DynamicMemory<T> {
    stack: Stack<T>,
    gc: Gc,
}

impl<T> DynamicMemory<T> {
    fn free(&mut self) -> Result<()> {
        for i in self.gc.position.iter(){
            self.stack.value.remove(*i);
        }
        Ok(())
    }
}
