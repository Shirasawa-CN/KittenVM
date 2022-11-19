#[cfg(test)]
mod tests {
    #[test]
    fn init() {
        use crate::vm::machine::KittenVM;
        #[allow(unused_imports)]
        use crate::*;
        let run: KittenVM = vm::machine::KittenVM::default();
    }
}
