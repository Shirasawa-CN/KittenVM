pub struct ConstPool<T>{
    Int: Vec<i128>,
    Nnint: Vec<usize>,
    Fract: Vec<f64>,
    Bool: Vec<bool>,
    Char: Vec<char>,
    Vector: Vec<Vec<T>>,
}

const CONST_POOL_INIT_CAP: usize = 8;

impl<T> ConstPool<T> {
    pub fn new(&self) -> Self{
        Self{
            Int: Vec::with_capacity(CONST_POOL_INIT_CAP),
            Nnint: Vec::with_capacity(CONST_POOL_INIT_CAP),
            Fract: Vec::with_capacity(CONST_POOL_INIT_CAP),
            Bool: Vec::with_capacity(CONST_POOL_INIT_CAP),
            Char: Vec::with_capacity(CONST_POOL_INIT_CAP),
            Vector: Vec::with_capacity(CONST_POOL_INIT_CAP),
        }
    }
}