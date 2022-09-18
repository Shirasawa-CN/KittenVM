use thiserror::Error;

#[derive(Error, Debug)]
pub enum Binary {
    //堆溢出
    #[error("Heap overflow")]
    HeapOverflow,
    //栈溢出
    #[error("Stack overflow")]
    StackOverflow,
}
