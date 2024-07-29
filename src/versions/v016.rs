use std::io::Read;

use super::{v015 as previous, FactorioVersion, RuntimeVersion};
use crate::{
    reader,
    reader::{FactorioNumber, FactorioReader},
};

pub type Latest = V016;

pub struct V016;

impl FactorioVersion for V016 {
    type PreviousVersion = previous::Latest;

    fn read_string(reader: &mut impl Read) -> std::io::Result<String> {
        // this is the exact same code as in 0.12, but Self::read_number works
        // differently since 0.14.14
        let length: u32 = Self::read_optimized_number(reader)?;

        reader::read_string(reader, length as _)
    }

    fn read_map<T: FactorioReader>(
        runtime_version: &RuntimeVersion,
        reader: &mut impl Read,
    ) -> std::io::Result<Vec<T>> {
        let amount = Self::read_optimized_number::<u32>(reader)?;

        let mut res = Vec::with_capacity(amount as _);
        for _ in 0..amount {
            let val = T::read(runtime_version, reader)?;
            res.push(val);
        }

        Ok(res)
    }

    fn read_allow_non_admin_debug_options(reader: &mut impl Read) -> std::io::Result<Option<bool>> {
        let res = u8::read_num(reader)?;
        Ok(Some(res != 0))
    }
}
