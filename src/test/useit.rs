#[cfg(test)]
mod tests {
    #[test]
    fn useit() {
        use crate::vm::machine::KittenVM;
        #[allow(unused_imports)]
        use crate::*;
        let mut run: KittenVM = vm::machine::KittenVM::default();
        let a = run.dynamic_memory.new_mem(vm::value_type::Type::int(36));
    }
}