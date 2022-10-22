use super::const_pool::ConstPool;
use super::dynamic_memory::DynamicMemory;
use crate::api::Mode;

enum Status{
    Working,
    Free,
}

pub struct KittenVM<T> {
    const_pool: ConstPool<T>,
    dynamic_memory: DynamicMemory<T>,
    status: Status,
    mode: Mode,
}
