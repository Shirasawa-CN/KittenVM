pub enum ErrorType {
    //关键字出错
    KeywordError,
    //堆溢出
    HeapOverflow,
    //栈溢出
    StackOverflow,
    //未知
    Unknown,
}

pub struct Info{
    number: String,
    errortype: ErrorType,
}
