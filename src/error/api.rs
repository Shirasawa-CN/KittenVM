use thiserror::Error;

#[derive(Error, Debug)]
pub enum Api {
    //找不到该指令
    #[error("The command cannot be found")]
    CommandCannotBeFound,
    //没有权限读取
    #[error("No permission to read")]
    NoPermissionToRead,
}
