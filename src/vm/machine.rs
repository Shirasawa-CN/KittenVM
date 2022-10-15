use super::const_pool::ConstPool;
use super::dynamic_memory::DynamicMemory;
pub struct KittenVM<T> {
    const_pool: ConstPool<T>,
    dynamic_memory: DynamicMemory<T>,
}
