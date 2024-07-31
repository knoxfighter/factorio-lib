use std::io::Read;

use crate::reader::FactorioNumber;
use crate::reader::read_optimized_num;

use super::{FactorioVersion, v015 as previous};

pub type Latest = V016;

pub struct V016;

impl FactorioVersion for V016 {
    type PreviousVersion = previous::Latest;

    fn read_array_length(reader: &mut impl Read) -> std::io::Result<u32> {
        read_optimized_num(reader)
    }

    fn read_allow_non_admin_debug_options(reader: &mut impl Read) -> std::io::Result<Option<bool>> {
        let res = u8::read_num(reader)?;
        Ok(Some(res != 0))
    }
}
