use super::const_pool::ConstPool;
use super::dynamic_memory::DynamicMemory;
use crate::api::Mode;
use crate::bitcode::KEYWORD;
use anyhow::Result;
use num_traits::NumOps;
use std::str::FromStr;
use std::sync::Arc;

enum Status {
    Working,
    Free,
}

pub struct KittenVM<T> {
    pub const_pool: ConstPool,
    pub dynamic_memory: DynamicMemory<T>,
    status: Status,
    pub mode: Mode,
}

impl<
        T: 'static
            + Clone
            + NumOps
            + std::ops::BitAnd
            + std::ops::BitXor
            + std::ops::BitOr
            + std::ops::Shl
            + std::ops::Shr
            + std::str::FromStr,
    > KittenVM<T>
where
    Arc<T>: NumOps,
    Arc<T>: std::ops::BitAnd<Output = Arc<T>>,
    Arc<T>: std::ops::BitXor<Output = Arc<T>>,
    Arc<T>: std::ops::BitOr<Output = Arc<T>>,
    Arc<T>: std::ops::Shl<Output = Arc<T>>,
    Arc<T>: std::ops::Shr<Output = Arc<T>>,
    <T as FromStr>::Err: Sync,
    <T as FromStr>::Err: Send,
    <T as FromStr>::Err: std::error::Error,
{
    /*
    匹配指令
    */
    pub async fn matcher(&mut self, code: String) -> Result<()> {
        if code.is_empty() {
            tracing::info!("code.is_empty()");
        }
        let code_info: Vec<&str> = code.split(' ').collect();
        let in_match = code_info[0];
        let _result: Result<(), anyhow::Error> = match in_match {
            "free" => self.dynamic_memory.free(),
            "add_gc" => self.dynamic_memory.add_gc(code_info[1].parse()?).await,
            "new" => self.const_pool.add(
                code_info[1].to_string(),
                self.dynamic_memory.new_mem(code_info[1].parse()?).await,
            ),
            "mov" => self
                .dynamic_memory
                .mov(code_info[1].parse()?, code_info[2].parse()?).await,
            "add" => self.dynamic_memory.add(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
            "sud" => self.dynamic_memory.sud(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
            "mul" => self.dynamic_memory.mul(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
            "div" => self.dynamic_memory.div(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
            "sll" => self.dynamic_memory.sll(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
            "sra" => self.dynamic_memory.sra(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
            "xor" => self.dynamic_memory.xor(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
            "or" => self.dynamic_memory.or(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
            "and" => self.dynamic_memory.and(
                code_info[1].parse()?,
                code_info[2].parse()?,
                code_info[3].parse()?,
            ).await,
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
