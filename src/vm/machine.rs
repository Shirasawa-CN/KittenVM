use super::const_pool::ConstPool;
use super::dynamic_memory::DynamicMemory;
use crate::api::Mode;
use crate::bitcode::KEYWORD;
use anyhow::Result;
use std::str::FromStr;

enum Status {
    Working,
    Free,
}

pub struct KittenVM<T> {
    pub const_pool: ConstPool<T>,
    pub dynamic_memory: DynamicMemory<T>,
    status: Status,
    pub mode: Mode,
}

impl<
        T: 'static
            + Clone
            + std::str::FromStr
            + std::ops::Add<Output = T>
            + std::ops::Sub<Output = T>
            + std::ops::Mul<Output = T>
            + std::ops::Div<Output = T>
            + std::ops::Shl<Output = T>
            + std::ops::Shr<Output = T>
            + std::ops::BitXor<Output = T>
            + std::ops::BitOr<Output = T>
            + std::ops::BitAnd<Output = T>,
    > KittenVM<T>
{
    /*
    匹配指令
    */
    pub fn matcher(&mut self, code: String) -> Result<()>
    where
        <T as FromStr>::Err: Send,
        <T as FromStr>::Err: std::error::Error,
        <T as FromStr>::Err: Sync,
    {
        if code.is_empty() {
            tracing::info!("code.is_empty()");
        }
        let code_info: Vec<&str> = code.split(' ').collect();
        let in_match = code_info[0];
        let result:Result<(), anyhow::Error> = match in_match {
            "free" => self.dynamic_memory.free(),
            "add_gc" => self.dynamic_memory.add_gc(code_info[1].parse()?),
            "new" => self
                .dynamic_memory
                .new(code_info[1].parse()?),
            "mov" => self
                .dynamic_memory
                .mov(code_info[1].parse()?, code_info[2].parse()?),
            "add" => self.dynamic_memory.add(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            "sud" => self.dynamic_memory.sud(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            "mul" => self.dynamic_memory.mul(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            "div" => self.dynamic_memory.div(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            "sll" => self.dynamic_memory.sll(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            "sra" => self.dynamic_memory.sra(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            "xor" => self.dynamic_memory.xor(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            "or" => self.dynamic_memory.or(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            "and" => self.dynamic_memory.and(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ),
            _ => {
                tracing::error!(
                    "Keyword not found\n All the current keywords are as follows{:#?}",
                    KEYWORD
                );
                return Err(anyhow::anyhow!("Keyword not found"));
            }
        };
        Ok(())
    }
}
